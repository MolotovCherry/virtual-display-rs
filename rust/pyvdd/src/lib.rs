#![allow(unsafe_op_in_unsafe_fn, non_snake_case)]
// ^ this module triggers this lint unfortunately, so it must be set to allow

use std::sync::OnceLock;
use std::{io::Write, sync::Mutex};

use driver_ipc::{Command, Dimen, Id, Mode, Monitor, RefreshRate};
use eyre::{bail, eyre, Result};
use pyo3::prelude::*;
use pyo3::{
    exceptions::{PyIndexError, PyKeyError, PyTypeError},
    types::{PyDict, PyList},
};
use tracing::{error, trace, trace_span};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use win_pipes::{NamedPipeClientOptions, NamedPipeClientWriter};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_SET_VALUE},
    RegKey,
};

static WRITER: OnceLock<Mutex<NamedPipeClientWriter>> = OnceLock::new();
static MONITORS: OnceLock<Mutex<Vec<Monitor>>> = OnceLock::new();
static REMOVAL_QUEUE: OnceLock<Mutex<Vec<Id>>> = OnceLock::new();

fn with_writer<R>(f: impl FnOnce(&mut NamedPipeClientWriter) -> R) -> Result<R> {
    let span = trace_span!("with_writer");
    let _guard = span.enter();

    let mut lock = WRITER.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;

    let r = f(&mut lock);

    Ok(r)
}

fn with_monitor<R>(id: Id, f: impl FnOnce(&mut Monitor) -> R) -> Result<R> {
    let span = trace_span!("with_monitor", id = id);
    let _guard = span.enter();

    let mut monitors = MONITORS.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;
    let monitor = monitors
        .iter_mut()
        .find(|m| m.id == id)
        .ok_or(eyre!("Monitor id {id} not found"))?;

    let r = f(monitor);

    Ok(r)
}

fn with_mode<R>(id: Id, idx: usize, f: impl FnOnce(&mut Mode) -> R) -> Result<R> {
    let span = trace_span!("with_mode", id = id, idx = idx);
    let _guard = span.enter();

    let mut monitors = MONITORS.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;
    let monitor = monitors
        .iter_mut()
        .find(|m| m.id == id)
        .ok_or(eyre!("Monitor id {id} not found"))?;

    let mode = monitor
        .modes
        .get_mut(idx)
        .ok_or(eyre!("Mode index {id} not found"))?;

    let r = f(mode);

    Ok(r)
}

fn remove_monitor(id: Id) -> Result<Monitor> {
    let span = trace_span!("remove_monitor", id = id);
    let _guard = span.enter();

    let mut monitors = MONITORS.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;
    let monitor = monitors
        .iter()
        .position(|m| m.id == id)
        .ok_or(eyre!("Monitor id {id} not found"))?;

    let monitor = monitors.remove(monitor);

    Ok(monitor)
}

fn remove_all_monitors() -> Result<()> {
    let span = trace_span!("remove_all_monitors");
    let _guard = span.enter();

    let mut monitors = MONITORS.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;
    monitors.clear();

    Ok(())
}

fn remove_mode(id: Id, idx: usize) -> Result<Mode> {
    let span = trace_span!("remove_mode", id = id, idx = idx);
    let _guard = span.enter();

    let mut monitors = MONITORS.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;
    let monitor = monitors
        .iter_mut()
        .find(|m| m.id == id)
        .ok_or(eyre!("Monitor id {id} not found"))?;

    if idx > monitor.modes.len() - 1 {
        bail!("Mode index {idx} out of bounds");
    }

    let mode = monitor.modes.remove(idx);

    Ok(mode)
}

#[pymodule]
fn pyvdd(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("PYVDD_LOG"))
        .init();

    m.add_class::<Monitors>()?;
    m.add_class::<MonitorWrapper>()?;
    m.add_class::<ModeWrapper>()?;
    m.add_class::<ModeList>()?;
    m.add_class::<ModeIterator>()?;
    m.add_class::<MonitorIterator>()?;

    Ok(())
}

#[derive(Debug)]
#[pyclass(frozen)]
struct Monitors;

#[allow(non_snake_case, clippy::unused_self)]
#[pymethods]
impl Monitors {
    #[new]
    fn new() -> PyResult<Self> {
        let span = trace_span!("Monitors::new()");
        let _guard = span.enter();

        // singleton check, just return self if already initialized
        if WRITER.get().is_some() {
            trace!("already instantiated");
            return Ok(Self);
        }

        trace!("instantiating");

        let (reader, mut writer) = NamedPipeClientOptions::new("virtualdisplaydriver")
            .wait()
            .access_duplex()
            .mode_message()
            .create()
            .map_err(|e| eyre!("Failed to connect to Virtual Display Driver; please ensure the driver is installed and working. Other program using the driver must also be closed, such as the Virtual Display Driver Control app.\n\nError: {e}"))?;

        let command = Command::RequestState;
        let command = serde_json::to_vec(&command).map_err(|e| eyre!("{e}"))?;
        writer.write_all(&command)?;

        let data = reader.read_full().map_err(|e| eyre!("{e}"))?;
        let command = serde_json::from_slice::<Command>(&data).map_err(|e| eyre!("{e}"))?;

        let Command::ReplyState(monitors) = command else {
            return Err(eyre!("invalid command reply: {command:?}").into());
        };

        trace!("received reply: {monitors:?}");

        MONITORS.set(Mutex::new(monitors)).unwrap();
        WRITER.set(Mutex::new(writer)).unwrap();
        REMOVAL_QUEUE.set(Mutex::new(Vec::new())).unwrap();

        Ok(Self)
    }

    /// Notify driver of changes
    fn notify(&self) -> PyResult<()> {
        let mut queue = REMOVAL_QUEUE
            .get()
            .unwrap()
            .lock()
            .map_err(|e| eyre!("{e}"))?;

        let removals = queue.drain(..).collect::<Vec<_>>();
        if !removals.is_empty() {
            let command = Command::DriverRemove(removals);
            let command = serde_json::to_vec(&command).map_err(|e| eyre!("{e}"))?;
            with_writer(|writer| writer.write_all(&command))??;
        }

        let monitors = MONITORS
            .get()
            .unwrap()
            .lock()
            .map_err(|e| eyre!("{e}"))?
            .clone();
        let command = Command::DriverNotify(monitors);
        trace!("command {command:?}");
        let data = serde_json::to_vec(&command).map_err(|e| eyre!("{e}"))?;

        with_writer(|writer| writer.write_all(&data))??;

        Ok(())
    }

    /// Remove multiple monitors by id
    fn remove(&self, list: Vec<Id>) -> PyResult<()> {
        // clear it out of our monitor list
        for &id in &list {
            remove_monitor(id)?;
        }

        let command = Command::DriverRemove(list);
        let command = serde_json::to_vec(&command).map_err(|e| eyre!("{e}"))?;
        with_writer(|writer| writer.write_all(&command))??;

        Ok(())
    }

    /// Remove all monitors
    fn remove_all(&self) -> PyResult<()> {
        // clear entire monitor list
        remove_all_monitors()?;

        let command = Command::DriverRemoveAll;
        let command = serde_json::to_vec(&command).map_err(|e| eyre!("{e}"))?;
        with_writer(|writer| writer.write_all(&command))??;

        Ok(())
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __repr__(&self) -> PyResult<String> {
        let monitors = MONITORS.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;
        Ok(format!("{:?}", *monitors))
    }

    fn __len__(&self) -> PyResult<usize> {
        let monitors = MONITORS.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;
        Ok(monitors.len())
    }

    #[allow(clippy::needless_pass_by_value, clippy::used_underscore_binding)]
    fn __iter__(_slf: Py<Self>) -> PyResult<MonitorIterator> {
        let monitors = MONITORS
            .get()
            .unwrap()
            .lock()
            .map_err(|e| eyre!("{e}"))?
            .clone()
            .into_iter()
            .map(|m| MonitorWrapper(m.id));

        let iterator = MonitorIterator {
            iter: Box::new(monitors),
        };

        Ok(iterator)
    }

    fn __delitem__(&self, id: u32) -> PyResult<()> {
        remove_monitor(id)?;

        REMOVAL_QUEUE
            .get()
            .unwrap()
            .lock()
            .map_err(|e| eyre!("{e}"))?
            .push(id);

        Ok(())
    }

    fn __getitem__(&self, id: u32) -> PyResult<MonitorWrapper> {
        let exists = MONITORS
            .get()
            .unwrap()
            .lock()
            .map_err(|e| eyre!("{e}"))?
            .iter()
            .any(|m| m.id == id);

        if !exists {
            return Err(PyIndexError::new_err(format!("Monitor id {id} not found")));
        }

        let monitor = MonitorWrapper(id);

        Ok(monitor)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn __setitem__(&self, py: Python, id: u32, data: PyObject) -> PyResult<()> {
        let span = trace_span!("Monitors::__setitem__()", id = id);
        let _guard = span.enter();

        let mut queue = REMOVAL_QUEUE
            .get()
            .unwrap()
            .lock()
            .map_err(|e| eyre!("{e}"))?;

        // make sure no ids we just added will be removed on notify
        queue.retain(|&qid| qid != id);

        let dict = data.downcast::<PyDict>(py)?;

        #[allow(clippy::redundant_closure_for_method_calls)]
        let name = dict
            .get_item("name")?
            // fix it if it's None
            .and_then(|o| {
                if o.is_none() {
                    None
                } else {
                    Some(o.extract::<String>())
                }
            })
            .transpose()?;

        #[allow(clippy::redundant_closure_for_method_calls)]
        let Some(enabled) = dict
            .get_item("enabled")?
            .map(|o| o.extract::<bool>())
            .transpose()?
        else {
            return Err(PyKeyError::new_err("enabled"));
        };

        #[allow(clippy::redundant_closure_for_method_calls)]
        let Some(py_modes) = dict
            .get_item("modes")?
            .map(|o| o.downcast::<PyList>())
            .transpose()?
        else {
            return Err(PyKeyError::new_err("modes"));
        };

        let mut modes = Vec::new();
        for mode in py_modes {
            let dict = mode.downcast::<PyDict>()?;

            #[allow(clippy::redundant_closure_for_method_calls)]
            let Some(width) = dict
                .get_item("width")?
                .map(|o| o.extract::<Dimen>())
                .transpose()?
            else {
                return Err(PyKeyError::new_err("width"));
            };

            #[allow(clippy::redundant_closure_for_method_calls)]
            let Some(height) = dict
                .get_item("height")?
                .map(|o| o.extract::<Dimen>())
                .transpose()?
            else {
                return Err(PyKeyError::new_err("height"));
            };

            #[allow(clippy::redundant_closure_for_method_calls)]
            let Some(refresh_rates) = dict
                .get_item("refresh_rates")?
                .map(|o| o.extract::<Vec<RefreshRate>>())
                .transpose()?
            else {
                return Err(PyKeyError::new_err("refresh_rates"));
            };

            modes.push(Mode {
                width,
                height,
                refresh_rates,
            });
        }

        let monitor = Monitor {
            id,
            name,
            enabled,
            modes,
        };

        trace!("computed {monitor:?}");

        let mut lock = MONITORS.get().unwrap().lock().map_err(|e| eyre!("{e}"))?;
        let pos = lock.iter().position(|mon| mon.id == id);
        if let Some(pos) = pos {
            trace!("replacing {pos}");
            _ = std::mem::replace(&mut lock[pos], monitor);
        } else {
            trace!("pushing");
            lock.push(monitor);
        }

        Ok(())
    }
}

impl Drop for Monitors {
    fn drop(&mut self) {
        // Persist data on drop.
        // Note that this will silently fail if unable to access or write to the registry
        // If it fails, check that you have access rights and that the path/key exists!

        let span = trace_span!("Monitors::drop()");
        let _guard = span.enter();

        trace!("drop");

        let Ok(monitors) = MONITORS.get().unwrap().lock() else {
            return;
        };

        trace!("got monitors OK");

        let Ok(data) = serde_json::to_string(&*monitors) else {
            return;
        };

        trace!(data = data, "convert to string OK");

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let Ok(vdisplay_reg) =
            hkcu.open_subkey_with_flags(r"SOFTWARE\VirtualDisplayDriver", KEY_SET_VALUE)
        else {
            return;
        };

        trace!(r"open subkey SOFTWARE\VirtualDisplayDriver OK");

        if let Err(e) = vdisplay_reg.set_value("data", &data) {
            error!("failed to persist: {e}");
            return;
        };

        trace!("save data OK");
    }
}

#[pyclass(frozen, name = "Monitor")]
struct MonitorWrapper(Id);

#[pymethods]
impl MonitorWrapper {
    #[getter]
    fn get_id(&self) -> PyResult<u32> {
        let id = with_monitor(self.0, |monitor| monitor.id)?;
        Ok(id)
    }

    #[getter]
    fn get_name(&self) -> PyResult<Option<String>> {
        let name = with_monitor(self.0, |monitor| monitor.name.clone())?;
        Ok(name)
    }

    #[setter]
    fn set_name(&self, name: Option<&str>) -> PyResult<()> {
        with_monitor(self.0, |monitor| monitor.name = name.map(ToOwned::to_owned))?;
        Ok(())
    }

    #[getter]
    fn get_enabled(&self) -> PyResult<bool> {
        let enabled = with_monitor(self.0, |monitor| monitor.enabled)?;
        Ok(enabled)
    }

    #[setter]
    fn set_enabled(&self, enabled: bool) -> PyResult<()> {
        with_monitor(self.0, |monitor| monitor.enabled = enabled)?;
        Ok(())
    }

    #[getter]
    fn get_modes(&self) -> ModeList {
        ModeList(self.0)
    }

    #[allow(clippy::needless_pass_by_value)]
    #[setter]
    fn set_modes(&self, py: Python, list: PyObject) -> PyResult<()> {
        let list = list.downcast::<PyList>(py)?;

        let mut modes = Vec::new();
        for obj in list {
            let dict = obj.downcast::<PyDict>()?;

            #[allow(clippy::redundant_closure_for_method_calls)]
            let Some(width) = dict
                .get_item("width")?
                .map(|o| o.extract::<Dimen>())
                .transpose()?
            else {
                return Err(PyKeyError::new_err("width"));
            };

            #[allow(clippy::redundant_closure_for_method_calls)]
            let Some(height) = dict
                .get_item("height")?
                .map(|o| o.extract::<Dimen>())
                .transpose()?
            else {
                return Err(PyKeyError::new_err("height"));
            };

            #[allow(clippy::redundant_closure_for_method_calls)]
            let Some(refresh_rates) = dict
                .get_item("refresh_rates")?
                .map(|o| o.extract::<Vec<RefreshRate>>())
                .transpose()?
            else {
                return Err(PyKeyError::new_err("refresh_rates"));
            };

            let mode = Mode {
                width,
                height,
                refresh_rates,
            };

            modes.push(mode);
        }

        with_monitor(self.0, |mon| mon.modes = modes)?;

        Ok(())
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __repr__(&self) -> PyResult<String> {
        let repr = with_monitor(self.0, |monitor| format!("{monitor:?}"))?;
        Ok(repr)
    }
}

#[pyclass(frozen)]
struct ModeList(Id);

#[pymethods]
impl ModeList {
    fn __getitem__(&self, idx: usize) -> PyResult<ModeWrapper> {
        let mode = with_monitor(self.0, |mon| idx < mon.modes.len())?;
        if !mode {
            return Err(PyIndexError::new_err("list index out of range"));
        }

        Ok(ModeWrapper { id: self.0, idx })
    }

    fn __delitem__(&self, idx: usize) -> PyResult<()> {
        remove_mode(self.0, idx)?;
        Ok(())
    }

    #[allow(clippy::needless_pass_by_value)]
    fn __setitem__(&self, py: Python, idx: usize, obj: PyObject) -> PyResult<()> {
        let dict = obj.downcast::<PyDict>(py)?;

        #[allow(clippy::redundant_closure_for_method_calls)]
        let Some(width) = dict
            .get_item("width")?
            .map(|o| o.extract::<Dimen>())
            .transpose()?
        else {
            return Err(PyKeyError::new_err("width"));
        };

        #[allow(clippy::redundant_closure_for_method_calls)]
        let Some(height) = dict
            .get_item("height")?
            .map(|o| o.extract::<Dimen>())
            .transpose()?
        else {
            return Err(PyKeyError::new_err("height"));
        };

        #[allow(clippy::redundant_closure_for_method_calls)]
        let Some(refresh_rates) = dict
            .get_item("refresh_rates")?
            .map(|o| o.extract::<Vec<RefreshRate>>())
            .transpose()?
        else {
            return Err(PyKeyError::new_err("refresh_rates"));
        };

        let mode = Mode {
            width,
            height,
            refresh_rates,
        };

        let valid = with_monitor(self.0, |mon| {
            if let Some(int_mode) = mon.modes.get_mut(idx) {
                _ = std::mem::replace(int_mode, mode);
            }

            idx < mon.modes.len()
        })?;

        if !valid {
            return Err(PyIndexError::new_err("list index out of range"));
        }

        Ok(())
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __repr__(&self) -> PyResult<String> {
        let repr = with_monitor(self.0, |mon| format!("{:?}", mon.modes))?;
        Ok(repr)
    }

    fn __len__(&self) -> PyResult<usize> {
        let len = with_monitor(self.0, |mon| mon.modes.len())?;
        Ok(len)
    }

    #[allow(clippy::needless_pass_by_value, clippy::used_underscore_binding)]
    fn __iter__(_slf: Py<Self>, py: Python) -> PyResult<ModeIterator> {
        let id = _slf.borrow(py).0;

        let modes = with_monitor(id, |mon| mon.modes.clone())?
            .into_iter()
            .enumerate()
            .map(move |(idx, _)| ModeWrapper { id, idx });

        let iter = ModeIterator {
            iter: Box::new(modes),
        };

        Ok(iter)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn __add__(&self, py: Python, obj: PyObject) -> PyResult<()> {
        if let Ok(list) = obj.downcast::<PyList>(py) {
            let mut modes = Vec::new();
            for dict in list {
                let width = dict.get_item("width")?.extract::<Dimen>()?;
                let height = dict.get_item("height")?.extract::<Dimen>()?;
                let refresh_rates = dict
                    .get_item("refresh_rates")?
                    .extract::<Vec<RefreshRate>>()?;

                let mode = Mode {
                    width,
                    height,
                    refresh_rates,
                };

                modes.push(mode);
            }

            with_monitor(self.0, |mon| mon.modes.extend(modes))?;
        } else if let Ok(dict) = obj.downcast::<PyDict>(py) {
            #[allow(clippy::redundant_closure_for_method_calls)]
            let width = dict
                .get_item("width")?
                .map(|o| o.extract::<Dimen>())
                .transpose()?
                .ok_or(PyTypeError::new_err("must be u32"))?;

            #[allow(clippy::redundant_closure_for_method_calls)]
            let height = dict
                .get_item("height")?
                .map(|o| o.extract::<Dimen>())
                .transpose()?
                .ok_or(PyTypeError::new_err("must be u32"))?;

            #[allow(clippy::redundant_closure_for_method_calls)]
            let refresh_rates = dict
                .get_item("refresh_rates")?
                .map(|o| o.extract::<Vec<RefreshRate>>())
                .transpose()?
                .ok_or(PyTypeError::new_err("must be u32"))?;

            let mode = Mode {
                width,
                height,
                refresh_rates,
            };

            with_monitor(self.0, |mon| mon.modes.push(mode))?;
        } else {
            return Err(PyTypeError::new_err("expected dict or list"));
        }

        Ok(())
    }
}

#[pyclass(frozen, name = "Mode")]
struct ModeWrapper {
    id: Id,
    idx: usize,
}

#[pymethods]
impl ModeWrapper {
    #[getter]
    fn get_width(&self) -> PyResult<u32> {
        let width = with_mode(self.id, self.idx, |mode| mode.width)?;
        Ok(width)
    }

    #[setter]
    fn set_width(&self, width: u32) -> PyResult<()> {
        with_mode(self.id, self.idx, |mode| mode.width = width)?;
        Ok(())
    }

    #[getter]
    fn get_height(&self) -> PyResult<u32> {
        let height = with_mode(self.id, self.idx, |mode| mode.height)?;
        Ok(height)
    }

    #[setter]
    fn set_height(&self, height: u32) -> PyResult<()> {
        with_mode(self.id, self.idx, |mode| mode.height = height)?;
        Ok(())
    }

    #[getter]
    fn get_refresh_rates(&self) -> PyResult<Vec<u32>> {
        let refresh_rates = with_mode(self.id, self.idx, |mode| mode.refresh_rates.clone())?;
        Ok(refresh_rates)
    }

    #[setter]
    fn set_refresh_rates(&self, refresh_rates: Vec<u32>) -> PyResult<()> {
        with_mode(self.id, self.idx, |mode| mode.refresh_rates = refresh_rates)?;
        Ok(())
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    fn __repr__(&self) -> PyResult<String> {
        let repr = with_mode(self.id, self.idx, |mode| format!("{mode:?}"))?;
        Ok(repr)
    }
}

#[pyclass]
struct MonitorIterator {
    iter: Box<dyn Iterator<Item = MonitorWrapper> + Send>,
}

#[pymethods]
impl MonitorIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<MonitorWrapper> {
        slf.iter.next()
    }
}

#[pyclass]
struct ModeIterator {
    iter: Box<dyn Iterator<Item = ModeWrapper> + Send>,
}

#[pymethods]
impl ModeIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<ModeWrapper> {
        slf.iter.next()
    }
}
