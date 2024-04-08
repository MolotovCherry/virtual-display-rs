#![allow(unsafe_op_in_unsafe_fn, non_snake_case, non_local_definitions)]
//       ^                                       ^
//       |                                       |-this one seems triggered by pymethods macro??
//       |-this module triggers this lint unfortunately, so it must be set to allow

use std::{
    borrow::{Borrow, Cow},
    collections::HashSet,
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use std::{fmt::Debug, sync::Mutex};

use driver_ipc::{
    ClientCommand, Dimen, DriverClient, EventCommand, Id, Mode, Monitor, RefreshRate,
};
use pyo3::prelude::*;
use pyo3::{
    exceptions::{PyIndexError, PyRuntimeError, PyTypeError},
    pyclass::boolean_struct::False,
    types::{DerefToPyAny, PyList, PyLong},
    DowncastIntoError, PyClass, PyTypeCheck,
};
use windows::Win32::{
    Foundation::{DuplicateHandle, DUPLICATE_SAME_ACCESS, HANDLE},
    System::{
        Threading::{GetCurrentProcess, GetCurrentThread},
        IO::CancelSynchronousIo,
    },
};

static INIT: AtomicBool = AtomicBool::new(false);

#[pymodule]
#[pyo3(name = "pyvdd")]
fn extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDriverClient>()?;
    m.add_class::<PyMonitor>()?;
    m.add_class::<PyMode>()?;

    Ok(())
}

#[derive(Copy, Clone)]
enum ListType {
    Monitor,
    Mode,
    RefreshRate,
}

impl Display for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListType::Monitor => write!(f, "Monitor"),
            ListType::Mode => write!(f, "Mode"),
            ListType::RefreshRate => write!(f, "u32"),
        }
    }
}

#[derive(Clone)]
#[pyclass(sequence)]
#[pyo3(name = "TypedList")]
struct PyTypedList {
    list: Py<PyList>,
    ty: ListType,
    iadd_flag: bool,
}

impl PyTypedList {
    fn new(py: Python, ty: ListType) -> Self {
        Self {
            list: PyList::empty_bound(py).into(),
            ty,
            iadd_flag: false,
        }
    }

    fn new_from_list(list: Py<PyList>, ty: ListType) -> Self {
        Self {
            list,
            ty,
            iadd_flag: false,
        }
    }

    fn get_index(&self, py: Python, index: isize) -> PyResult<usize> {
        let len = self.__len__(py);

        let Some(abs_index) = index.checked_abs() else {
            return Err(PyIndexError::new_err("list index out of bounds"));
        };
        let abs_index: usize = abs_index.try_into()?;
        let abs_index = abs_index.saturating_sub(1);

        // convert index to appropriate index
        let index = if index >= 0 {
            let len: isize = len.try_into()?;
            if index >= len {
                return Err(PyIndexError::new_err("list index out of bounds"));
            }

            // this is > 0, no signs are lost
            #[allow(clippy::cast_sign_loss)]
            {
                index as usize
            }
        } else if let Some(index) = (len.saturating_sub(1)).checked_sub(abs_index) {
            index
        } else {
            return Err(PyIndexError::new_err("list index out of bounds"));
        };

        Ok(index)
    }
}

#[pymethods]
impl PyTypedList {
    fn __len__(&self, py: Python) -> usize {
        self.list.bind(py).len()
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        self.list.bind(py).repr().map(|s| s.to_string())
    }

    fn __str__(&self, py: Python) -> PyResult<String> {
        self.list.bind(py).str().map(|s| s.to_string())
    }

    #[allow(clippy::needless_pass_by_value)]
    fn __iadd__(&mut self, py: Python, obj: PyObject) -> PyResult<()> {
        pytypedlist_append_pyobject(py, self, obj)?;
        self.iadd_flag = true;
        Ok(())
    }

    fn __setitem__(&self, py: Python, index: isize, item: PyObject) -> PyResult<()> {
        let index = self.get_index(py, index)?;

        let item_b = item.bind(py);
        let ty = item.bind(py).get_type();

        let inner = self.list.bind(py);

        let is_valid = match self.ty {
            ListType::Monitor => {
                if let Ok(mon) = item_b.downcast_exact::<PyMonitor>().map(Bound::borrow) {
                    for (i, item) in inner.iter().enumerate() {
                        let item = item.downcast_exact::<PyMonitor>()?.borrow();
                        if item.id == mon.id && i != index {
                            return Err(PyRuntimeError::new_err(format!(
                                "monitors list already contains a Monitor with id {}",
                                item.id,
                            )));
                        }
                    }

                    true
                } else {
                    false
                }
            }

            ListType::Mode => {
                if let Ok(mode) = item_b.downcast_exact::<PyMode>().map(Bound::borrow) {
                    for (i, item) in inner.iter().enumerate() {
                        let item = item.downcast_exact::<PyMode>()?.borrow();
                        if item.width == mode.width && item.height == mode.height && i != index {
                            return Err(PyRuntimeError::new_err(format!(
                                "modes list already contains a Mode {}x{}",
                                item.width, item.height,
                            )));
                        }
                    }

                    true
                } else {
                    false
                }
            }

            ListType::RefreshRate => {
                if let Ok(rr) = item_b.extract::<u32>() {
                    for (i, item) in inner.iter().enumerate() {
                        let item = item.extract::<RefreshRate>()?;

                        if item == rr && index != i {
                            return Err(PyRuntimeError::new_err(format!(
                                "refresh_rates list already contains refresh rate {rr}"
                            )));
                        }
                    }

                    true
                } else {
                    false
                }
            }
        };

        if !is_valid {
            return Err(PyTypeError::new_err(format!(
                "expected {}, got {}",
                self.ty,
                ty.name()?,
            )));
        }

        self.list.bind(py).set_item(index, item)
    }

    fn __getitem__(&self, py: Python, index: isize) -> PyResult<PyObject> {
        let index = self.get_index(py, index)?;
        self.list.bind(py).get_item(index).map(Into::into)
    }

    fn __contains__(&self, py: Python, obj: PyObject) -> PyResult<bool> {
        self.list.bind(py).contains(obj)
    }

    fn __delitem__(&self, py: Python, index: isize) -> PyResult<()> {
        let index = self.get_index(py, index)?;
        self.list.bind(py).del_item(index)
    }
}

impl TryFrom<PyTypedList> for Py<PyTypedList> {
    type Error = PyErr;

    fn try_from(value: PyTypedList) -> Result<Self, Self::Error> {
        Python::with_gil(|py| Py::new(py, value))
    }
}

trait IntoPyListIter {
    fn iter_ref<'py, T: PyClass + Clone>(
        &self,
        py: Python<'py>,
    ) -> impl Iterator<Item = Result<PyRef<'py, T>, DowncastIntoError<'py>>>;

    fn iter_ref_mut<'py, T: PyClass<Frozen = False> + Clone>(
        &self,
        py: Python<'py>,
    ) -> impl Iterator<Item = Result<PyRefMut<'py, T>, DowncastIntoError<'py>>>;

    // fn iter_bound<'py, T: PyClass + Clone>(
    //     &self,
    //     py: Python<'py>,
    // ) -> impl Iterator<Item = Result<Bound<'py, T>, DowncastIntoError<'py>>>;

    // fn iter_extract<'py, P: PyTypeCheck + DerefToPyAny + FromPyObject<'py>>(
    //     &self,
    //     py: Python<'py>,
    // ) -> impl Iterator<Item = Result<Result<P, PyErr>, DowncastIntoError<'py>>>;

    fn iter_py_extract<'py, P: PyTypeCheck + DerefToPyAny, E: FromPyObject<'py>>(
        &self,
        py: Python<'py>,
    ) -> impl Iterator<Item = Result<Result<E, PyErr>, DowncastIntoError<'py>>>;
}

impl IntoPyListIter for Py<PyTypedList> {
    fn iter_ref<'py, T: PyClass + Clone>(
        &self,
        py: Python<'py>,
    ) -> impl Iterator<Item = Result<PyRef<'py, T>, DowncastIntoError<'py>>> {
        self.bind(py)
            .borrow()
            .list
            .bind(py)
            .iter()
            .map(|i| i.downcast_into().map(|b| b.borrow()))
    }

    fn iter_ref_mut<'py, T: PyClass<Frozen = False> + Clone>(
        &self,
        py: Python<'py>,
    ) -> impl Iterator<Item = Result<PyRefMut<'py, T>, DowncastIntoError<'py>>> {
        self.bind(py)
            .borrow()
            .list
            .bind(py)
            .iter()
            .map(|i| i.downcast_into().map(|b| b.borrow_mut()))
    }

    // fn iter_bound<'py, T: PyClass + Clone>(
    //     &self,
    //     py: Python<'py>,
    // ) -> impl Iterator<Item = Result<Bound<'py, T>, DowncastIntoError<'py>>> {
    //     self.bind(py)
    //         .borrow()
    //         .list
    //         .bind(py)
    //         .iter()
    //         .map(PyAnyMethods::downcast_into)
    // }

    // fn iter_extract<'py, P: PyTypeCheck + DerefToPyAny + FromPyObject<'py>>(
    //     &self,
    //     py: Python<'py>,
    // ) -> impl Iterator<Item = Result<Result<P, PyErr>, DowncastIntoError<'py>>> {
    //     self.bind(py)
    //         .borrow()
    //         .list
    //         .bind(py)
    //         .iter()
    //         .map(|i| i.downcast_into::<P>().map(|i| i.extract::<P>()))
    // }

    fn iter_py_extract<'py, P: PyTypeCheck + DerefToPyAny, E: FromPyObject<'py>>(
        &self,
        py: Python<'py>,
    ) -> impl Iterator<Item = Result<Result<E, PyErr>, DowncastIntoError<'py>>> {
        self.bind(py)
            .borrow()
            .list
            .bind(py)
            .iter()
            .map(|i| i.downcast_into::<P>().map(|i| i.extract::<E>()))
    }
}

/// The main driver client. As long as this is open, it will remain connected to the monitor.
/// Only one instance is allowed
#[pyclass]
#[pyo3(name = "DriverClient")]
struct PyDriverClient {
    client: DriverClient,
    thread_registry: Arc<Mutex<Option<HANDLE>>>,
    /// The list of monitors
    /// Sig: list[Monitor]
    #[pyo3(get)]
    monitors: Py<PyTypedList>,
}

#[pymethods]
impl PyDriverClient {
    #[new]
    fn new(py: Python) -> PyResult<Self> {
        if INIT.swap(true, Ordering::Relaxed) {
            return Err(PyRuntimeError::new_err(
                "Only one instance may exist at any time",
            ));
        }

        let mut client = DriverClient::new()?;
        client.refresh_state()?;

        let monitors = state_to_pytypedlist(py, client.monitors())?;

        let slf = Self {
            client,
            monitors,
            thread_registry: Arc::new(Mutex::new(None)),
        };

        Ok(slf)
    }

    #[allow(clippy::needless_pass_by_value)]
    #[setter]
    fn set_monitors(&mut self, py: Python, obj: PyObject) -> PyResult<()> {
        let mut inner = self.monitors.bind(py).borrow_mut();

        if inner.iadd_flag {
            inner.iadd_flag = false;
        } else {
            let list = PyList::empty_bound(py);
            pylist_append_pyobject(py, list.as_borrowed(), obj, ListType::Monitor)?;
            self.monitors =
                PyTypedList::new_from_list(list.into(), ListType::Monitor).try_into()?;
        }

        Ok(())
    }

    /// Persist monitor configuration for user
    /// Sig: perist()
    fn persist(&mut self, py: Python) -> PyResult<()> {
        let state = pytypedlist_to_state(py, &self.monitors)?;
        self.client.set_monitors(&state)?;

        self.client.persist()?;

        Ok(())
    }

    /// Request a list of latest driver changes
    /// Sig: get_state() -> list[Monitor]
    fn get_state(&mut self, py: Python) -> PyResult<Py<PyList>> {
        self.client.refresh_state()?;

        let monitors = state_to_pylist(py, self.client.monitors())?;

        Ok(monitors)
    }

    /// Send notification to driver of changes
    /// Sig: notify()
    fn notify(&mut self, py: Python) -> PyResult<()> {
        let state = pytypedlist_to_state(py, &self.monitors)?;
        self.client.set_monitors(&state)?;

        self.client.notify()?;

        Ok(())
    }

    /// Get notified of other clients changing driver configuration
    /// Sig: receive(Callable[list[Monitor], None])
    fn receive(&self, callback: PyObject) {
        // cancel the receiver internally if called again
        {
            let lock = self.thread_registry.lock().unwrap().take();
            if let Some(thread) = lock {
                unsafe {
                    _ = CancelSynchronousIo(thread);
                }
            }
        }

        let registry = self.thread_registry.clone();
        self.client.set_receiver(
            // init - store thread handle for later
            Some(move || {
                let mut lock = registry.lock().unwrap();

                let pseudo_handle = unsafe { GetCurrentThread() };
                let current_process = unsafe { GetCurrentProcess() };

                let mut thread_handle = HANDLE::default();
                unsafe {
                    _ = DuplicateHandle(
                        current_process,
                        pseudo_handle,
                        current_process,
                        &mut thread_handle,
                        0,
                        false,
                        DUPLICATE_SAME_ACCESS,
                    );
                }

                *lock = Some(thread_handle);
            }),
            // cb
            move |command| {
                if let ClientCommand::Event(EventCommand::Changed(data)) = command {
                    Python::with_gil(|py| {
                        let state = state_to_pylist(py, &data);
                        let Ok(state) = state else {
                            return;
                        };

                        if let Err(e) = callback.call1(py, (state,)) {
                            e.print(py);
                        }
                    });
                }
            },
        );
    }

    /// Find id of monitor based on query. Query is a string containing monitor's name or id
    /// Sig: find_id(query: str) -> Optional[int]
    fn find_id(&self, query: &str) -> Option<Id> {
        self.client.find_id(query).ok()
    }

    /// Find a monitor by Id
    /// Sig: find_monitor(query: int | str) -> Optional[Monitor]
    #[allow(clippy::needless_pass_by_value)]
    fn find_monitor(&self, py: Python, query: PyObject) -> PyResult<Option<Py<PyMonitor>>> {
        let iter = self.monitors.iter_ref::<PyMonitor>(py);

        let query_b = query.bind(py);
        let id = query_b.extract::<u32>();
        let query = query_b.extract::<String>();

        if id.is_err() && query.is_err() {
            let ty = query_b.get_type();
            return Err(PyTypeError::new_err(format!(
                "expected u32|str, got {}",
                ty.name()?,
            )));
        }

        for monitor in iter {
            let monitor = monitor?;
            let b = monitor.borrow();

            if let Ok(id) = id {
                if b.id == id {
                    return Ok(Some(monitor.into()));
                }
            }

            if let Ok(query) = query.as_deref() {
                if b.name.as_deref().is_some_and(|name| name == query) {
                    return Ok(Some(monitor.into()));
                }
            }
        }

        Ok(None)
    }

    /// Get the closest available free ID. Note that if internal state is stale, this may result in a duplicate ID
    /// which the driver will ignore when you notify it of changes
    /// Sig: new_id(preferred_id: Optional[int] = None) -> Optional[int]
    fn new_id(&mut self, py: Python, preferred_id: Option<Id>) -> PyResult<Option<Id>> {
        let state = pytypedlist_to_state(py, &self.monitors)?;
        // by setting this, we can ensure it's up to date before trying to get the new id
        if self.client.set_monitors(&state).is_err() {
            return Ok(None);
        }
        Ok(self.client.new_id(preferred_id).ok())
    }

    /// Remove monitors by id
    /// Sig: remove(list[int])
    #[allow(clippy::needless_pass_by_value)]
    fn remove(&mut self, py: Python, ids: Vec<Id>) -> PyResult<()> {
        let monitors = self.monitors.iter_ref::<PyMonitor>(py);

        let mut remove_pos = Vec::new();
        for (i, monitor) in monitors.enumerate() {
            let monitor = monitor?;
            if ids.contains(&monitor.id) {
                remove_pos.push(i);
            }
        }

        let monitors = self.monitors.bind(py);
        for (i, pos) in remove_pos.iter().enumerate() {
            monitors.del_item(pos - i)?;
        }

        // keep internal state of client consistent
        let state = pytypedlist_to_state(py, &self.monitors)?;
        self.client.set_monitors(&state)?;
        Ok(())
    }

    /// Enable monitors by id or query. Query is a string containing an id or name
    /// Sig: set_enabled(queries: list[int | str], enabled: bool)
    #[allow(clippy::needless_pass_by_value)]
    fn set_enabled(&mut self, py: Python, queries: Vec<PyObject>, enabled: bool) -> PyResult<()> {
        let queries = queries
            .into_iter()
            .map(|query| {
                let query = query.bind(py);

                (
                    query.extract::<Id>(),
                    query.extract::<String>(),
                    query.get_type().name().map(Cow::into_owned),
                )
            })
            .collect::<Vec<_>>();

        let monitors = self.monitors.iter_ref_mut::<PyMonitor>(py);

        for mon in monitors {
            let mut mon = mon?;

            for (id, query, name) in &queries {
                if id.is_err() && query.is_err() {
                    let name = name.as_deref().map_err(|e| e.clone_ref(py))?;

                    return Err(PyTypeError::new_err(format!(
                        "expected u32|str, got {name}",
                    )));
                }

                if let &Ok(id) = id {
                    if mon.id == id {
                        mon.enabled = enabled;
                    }
                } else if let Ok(query) = query.as_deref() {
                    if mon.name.as_deref().is_some_and(|name| name == query) {
                        mon.enabled = enabled;
                    }
                }
            }
        }

        // keep internal state of client consistent
        let state = pytypedlist_to_state(py, &self.monitors)?;
        self.client.set_monitors(&state)?;
        Ok(())
    }

    fn __repr__(&self, py: Python) -> String {
        format!("DriverClient {{ monitors: {:?} }}", self.monitors.bind(py))
    }
}

impl Debug for PyDriverClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let PyDriverClient { monitors, .. } = self;
        f.debug_struct("DriverClient")
            .field("monitors", &monitors)
            .finish()
    }
}

impl Drop for PyDriverClient {
    fn drop(&mut self) {
        INIT.store(false, Ordering::Relaxed);
    }
}

/// A Monitor. Each monitor's id must be unique from the others.
#[derive(Clone)]
#[pyclass]
#[pyo3(name = "Monitor")]
struct PyMonitor {
    /// The monitor's id. Must be unique from all other monitors
    /// Sig: id: int
    #[pyo3(get, set)]
    id: Id,
    /// The monitor's name. May be empty or specified
    /// Sig: name: Optional[str]
    #[pyo3(get, set)]
    name: Option<String>,
    /// Whether the monitor is enabled or not
    /// Sig: enabled: bool
    #[pyo3(get, set)]
    enabled: bool,
    /// The monitor modes (resolution and refresh rates)
    /// Sig: modes: list[Mode]
    #[pyo3(get)]
    modes: Py<PyTypedList>,
}

#[pymethods]
impl PyMonitor {
    #[new]
    fn new(py: Python) -> PyResult<PyMonitor> {
        let inst = Self {
            id: 0,
            name: None,
            enabled: false,
            modes: PyTypedList::new(py, ListType::Mode).try_into()?,
        };

        Ok(inst)
    }

    #[allow(clippy::needless_pass_by_value)]
    #[setter]
    fn set_modes(&mut self, py: Python, obj: PyObject) -> PyResult<()> {
        let mut inner = self.modes.bind(py).borrow_mut();

        if inner.iadd_flag {
            inner.iadd_flag = false;
        } else {
            let list = PyList::empty_bound(py);
            pylist_append_pyobject(py, list.as_borrowed(), obj, ListType::Mode)?;
            self.modes = PyTypedList::new_from_list(list.into(), ListType::Mode).try_into()?;
        }

        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

impl TryFrom<PyMonitor> for Py<PyMonitor> {
    type Error = PyErr;

    fn try_from(value: PyMonitor) -> Result<Self, Self::Error> {
        Python::with_gil(|py| Py::new(py, value))
    }
}

impl Debug for PyMonitor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Python::with_gil(|py| {
            let PyMonitor {
                id,
                name,
                enabled,
                modes,
            } = self;

            let modes = modes
                .iter_ref::<PyMode>(py)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| std::fmt::Error)?;

            f.debug_struct("Monitor")
                .field("id", &id)
                .field("name", &name)
                .field("enabled", &enabled)
                .field("modes", &modes)
                .finish()
        })
    }
}

/// A monitor mode which represents a resolution and associated refresh rates.
/// Each mode must be unique from other modes (a unique width x height),
/// and the refresh rates must also be unique per mode
#[derive(Clone)]
#[pyclass]
#[pyo3(name = "Mode")]
struct PyMode {
    /// The mode width
    /// Sig: width: int
    #[pyo3(get, set)]
    width: Dimen,
    /// The mode height
    /// Sig: height: int
    #[pyo3(get, set)]
    height: Dimen,
    /// The mode's refresh rates. Each refresh rate must be unique. No duplicates allowed
    /// Sig: refresh_rates: list[int]
    #[pyo3(get)]
    refresh_rates: Py<PyTypedList>,
}

impl TryFrom<PyMode> for Py<PyMode> {
    type Error = PyErr;

    fn try_from(value: PyMode) -> Result<Self, Self::Error> {
        Python::with_gil(|py| Py::new(py, value))
    }
}

impl Debug for PyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Python::with_gil(|py| {
            let PyMode {
                width,
                height,
                refresh_rates,
            } = self;

            let refresh_rates = refresh_rates
                .iter_py_extract::<PyLong, RefreshRate>(py)
                .collect::<Result<Result<Vec<_>, _>, _>>()
                .map_err(|_| std::fmt::Error)?
                .map_err(|_| std::fmt::Error)?;

            f.debug_struct("Mode")
                .field("width", &width)
                .field("height", &height)
                .field("refresh_rates", &refresh_rates)
                .finish()
        })
    }
}

#[pymethods]
impl PyMode {
    #[new]
    fn new(py: Python) -> PyResult<PyMode> {
        let inst = Self {
            width: 0,
            height: 0,
            refresh_rates: PyTypedList::new(py, ListType::RefreshRate).try_into()?,
        };

        Ok(inst)
    }

    #[allow(clippy::needless_pass_by_value)]
    #[setter]
    fn set_refresh_rates(&mut self, py: Python, obj: PyObject) -> PyResult<()> {
        let mut inner = self.refresh_rates.bind(py).borrow_mut();

        if inner.iadd_flag {
            inner.iadd_flag = false;
        } else {
            let list = PyList::empty_bound(py);
            pylist_append_pyobject(py, list.as_borrowed(), obj, ListType::RefreshRate)?;
            self.refresh_rates =
                PyTypedList::new_from_list(list.into(), ListType::RefreshRate).try_into()?;
        }

        Ok(())
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

fn state_to_pylist(py: Python, monitors: &[Monitor]) -> PyResult<Py<PyList>> {
    let py_state = PyList::empty_bound(py);

    for monitor in monitors {
        let modes = PyList::empty_bound(py);

        for mode in &monitor.modes {
            let py_refresh_rates = PyList::new_bound(py, &mode.refresh_rates);

            let mode: Py<PyMode> = PyMode {
                width: mode.width,
                height: mode.height,
                refresh_rates: PyTypedList::new_from_list(
                    py_refresh_rates.into(),
                    ListType::RefreshRate,
                )
                .try_into()?,
            }
            .try_into()?;

            modes.append(mode)?;
        }

        let monitor: Py<PyMonitor> = PyMonitor {
            id: monitor.id,
            name: monitor.name.clone(),
            enabled: monitor.enabled,
            modes: PyTypedList::new_from_list(modes.into(), ListType::Mode).try_into()?,
        }
        .try_into()?;

        py_state.append(monitor)?;
    }

    Ok(py_state.into())
}

fn state_to_pytypedlist(py: Python, monitors: &[Monitor]) -> PyResult<Py<PyTypedList>> {
    let py_state = state_to_pylist(py, monitors)?;

    let typed_list = PyTypedList::new_from_list(py_state, ListType::Monitor).try_into()?;

    Ok(typed_list)
}

fn pytypedlist_to_state(py: Python, monitors: &Py<PyTypedList>) -> PyResult<Vec<Monitor>> {
    let mut state = Vec::new();

    let monitors = monitors.iter_ref::<PyMonitor>(py);

    for py_monitor in monitors {
        let mut modes = Vec::new();

        let py_monitor = py_monitor?;
        let py_modes = py_monitor.modes.iter_ref::<PyMode>(py);

        for mode in py_modes {
            let mode = mode?;

            let refresh_rates = mode
                .refresh_rates
                .iter_py_extract::<PyLong, RefreshRate>(py)
                .collect::<Result<Result<Vec<_>, _>, _>>()??;

            modes.push(Mode {
                width: mode.width,
                height: mode.height,
                refresh_rates,
            });
        }

        state.push(Monitor {
            id: py_monitor.id,
            name: py_monitor.name.clone(),
            enabled: py_monitor.enabled,
            modes,
        });
    }

    Ok(state)
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
fn pylist_append_pyobject(
    py: Python,
    inner: Borrowed<'_, '_, PyList>,
    obj: PyObject,
    list_ty: ListType,
) -> PyResult<()> {
    let obj = obj.bind(py);

    let mut ty = obj.get_type();
    let user_pylist = obj.downcast_exact::<PyList>();
    let py_monitor = obj.downcast_exact::<PyMonitor>();
    let py_mode = obj.downcast_exact::<PyMode>();
    let py_long = obj.extract::<u32>();

    let mut is_ok = true;
    match list_ty {
        ListType::Monitor => {
            if let Ok(mon) = py_monitor {
                let b = mon.borrow();

                for item in inner.iter() {
                    let item = item.downcast_exact::<PyMonitor>()?.borrow();
                    if item.id == b.id {
                        return Err(PyRuntimeError::new_err(format!(
                            "monitors list already contains a Monitor with id {}",
                            b.id,
                        )));
                    }
                }

                inner.append(mon)?;
                return Ok(());
            } else if let Ok(user_list) = user_pylist {
                let mut used = HashSet::new();
                let mut buf = Vec::new();

                for item in user_list.iter() {
                    if let Ok(mon) = item.clone().downcast_into_exact::<PyMonitor>() {
                        let b = mon.downcast_exact::<PyMonitor>()?.borrow();

                        if !used.insert(b.id) {
                            return Err(PyRuntimeError::new_err(format!(
                                "list of Monitor already contains a Monitor with id {}",
                                b.id
                            )));
                        }

                        for item in inner.iter() {
                            let item = item.downcast_exact::<PyMonitor>()?.borrow();
                            if item.id == b.id {
                                return Err(PyRuntimeError::new_err(format!(
                                    "monitors list already contains a Monitor with id {}",
                                    b.id,
                                )));
                            }
                        }

                        buf.push(mon);
                        continue;
                    }

                    is_ok = false;
                    ty = item.get_type();
                    break;
                }

                if is_ok {
                    for rr in buf {
                        inner.append(rr)?;
                    }
                }
            }
        }

        ListType::Mode => {
            if let Ok(mode) = py_mode {
                // you can't enter duplicate refresh rates since we already checked that
                // but inner might have it still

                let m = mode.borrow();
                for item in inner.iter() {
                    let item = item.downcast_exact::<PyMode>().unwrap().borrow();
                    if item.width == m.width && item.height == m.height {
                        return Err(PyRuntimeError::new_err(format!(
                            "modes list already contains a Mode {}x{}",
                            m.width, m.height,
                        )));
                    }
                }

                inner.append(mode)?;
                return Ok(());
            } else if let Ok(user_list) = user_pylist {
                let mut used = HashSet::new();
                let mut buf = Vec::new();

                for item in user_list.iter() {
                    if let Ok(mode) = item.downcast_exact::<PyMode>() {
                        let b = mode.borrow();

                        if !used.insert((b.width, b.height)) {
                            return Err(PyRuntimeError::new_err(format!(
                                "list of Mode already contains a Mode {}x{}",
                                b.width, b.height
                            )));
                        }

                        for item in inner.iter() {
                            let item = item.downcast_exact::<PyMode>().unwrap().borrow();
                            if item.width == b.width && item.height == b.height {
                                return Err(PyRuntimeError::new_err(format!(
                                    "modes list already contains a Mode {}x{}",
                                    b.width, b.height,
                                )));
                            }
                        }

                        buf.push(mode.clone());
                        continue;
                    }

                    is_ok = false;
                    ty = item.get_type();
                    break;
                }

                if is_ok {
                    for rr in buf {
                        inner.append(rr)?;
                    }
                }
            }
        }

        ListType::RefreshRate => {
            if let Ok(rr) = py_long {
                for item in inner.iter() {
                    let item = item.extract::<RefreshRate>().unwrap();
                    if item == rr {
                        return Err(PyRuntimeError::new_err(format!(
                            "refresh_rates list already contains refresh rate {item}"
                        )));
                    }
                }

                inner.append(rr)?;
                return Ok(());
            } else if let Ok(user_list) = user_pylist {
                let mut buf = Vec::new();

                for item in user_list.iter() {
                    if let Ok(rr) = item.extract::<RefreshRate>() {
                        if buf.contains(&rr) {
                            return Err(PyRuntimeError::new_err(format!(
                                "list of refresh rates already contains refresh rate {rr}"
                            )));
                        }

                        for item in inner.iter() {
                            let item = item.extract::<RefreshRate>().unwrap();
                            if item == rr {
                                return Err(PyRuntimeError::new_err(format!(
                                    "refresh_rates list already contains refresh rate {rr}"
                                )));
                            }
                        }

                        buf.push(rr);
                        continue;
                    }

                    is_ok = false;
                    ty = item.get_type();
                    break;
                }

                if is_ok {
                    for rr in buf {
                        inner.append(rr)?;
                    }
                }
            }
        }
    }

    if is_ok {
        Ok(())
    } else {
        Err(PyTypeError::new_err(format!(
            "expected {}, got {}",
            list_ty,
            ty.name()?,
        )))
    }
}

#[allow(clippy::needless_pass_by_value)]
fn pytypedlist_append_pyobject(
    py: Python,
    py_typed_list: &PyTypedList,
    obj: PyObject,
) -> PyResult<()> {
    pylist_append_pyobject(
        py,
        py_typed_list.list.bind_borrowed(py),
        obj,
        py_typed_list.ty,
    )
}
