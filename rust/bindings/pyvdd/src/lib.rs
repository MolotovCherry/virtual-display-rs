#![allow(unsafe_op_in_unsafe_fn, non_snake_case, non_local_definitions)]
//       ^                                       ^
//       |                                       |-this one seems triggered by pymethods macro??
//       |-this module triggers this lint unfortunately, so it must be set to allow

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{
    fmt::{Debug, Display},
    sync::Mutex,
};

use driver_ipc::{
    ClientCommand, Dimen, DriverClient, EventCommand, Id, Mode, Monitor, RefreshRate,
};
use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;
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

#[pyclass]
#[pyo3(name = "RefreshRate")]
struct PyRefreshRate(RefreshRate);

#[pymethods]
impl PyRefreshRate {
    fn __repr__(&self) -> String {
        self.__str__()
    }

    fn __str__(&self) -> String {
        self.to_string()
    }
}

impl Debug for PyRefreshRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PyRefreshRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// data stored in container
enum Data {
    Monitors(Vec<Py<PyMonitor>>),
    Modes(Vec<Py<PyMode>>),
    RefreshRates(Vec<Py<PyRefreshRate>>),
}

#[pyclass(sequence)]
#[pyo3(name = "Container")]
struct PyContainer(Data);

#[pymethods]
impl PyContainer {
    fn __len__(&self) -> usize {
        match &self.0 {
            Data::Monitors(m) => m.len(),
            Data::Modes(m) => m.len(),
            Data::RefreshRates(r) => r.len(),
        }
    }

    fn __getitem__(&self, index: isize) -> PyResult<PyObject> {
        let index = self.get_index(index)?;

        let item = match &self.0 {
            Data::Monitors(m) => m[index].as_any().clone(),
            Data::Modes(m) => m[index].as_any().clone(),
            Data::RefreshRates(r) => r[index].as_any().clone(),
        };

        Ok(item)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn __setitem__(&mut self, py: Python, index: isize, obj: PyObject) -> PyResult<()> {
        let index = self.get_index(index)?;

        match &mut self.0 {
            Data::Monitors(m) => m[index] = obj.extract(py)?,
            Data::Modes(m) => m[index] = obj.extract(py)?,
            Data::RefreshRates(r) => {
                r[index] = {
                    let int: RefreshRate = obj.extract(py)?;
                    Py::new(py, PyRefreshRate(int))?
                }
            }
        }

        Ok(())
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        let index = self.get_index(index)?;

        match &mut self.0 {
            Data::Monitors(m) => {
                m.remove(index);
            }
            Data::Modes(m) => {
                m.remove(index);
            }
            Data::RefreshRates(r) => {
                r.remove(index);
            }
        }

        Ok(())
    }

    #[allow(clippy::needless_pass_by_value)]
    fn __iadd__(&mut self, py: Python, obj: PyObject) -> PyResult<()> {
        match &mut self.0 {
            Data::Monitors(m) => m.push(obj.extract(py)?),
            Data::Modes(m) => m.push(obj.extract(py)?),
            Data::RefreshRates(r) => {
                let int: RefreshRate = obj.extract(py)?;
                let obj = Py::new(py, PyRefreshRate(int))?;
                r.push(obj);
            }
        }

        Ok(())
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

impl PyContainer {
    fn get_index(&self, index: isize) -> PyResult<usize> {
        let len = match &self.0 {
            Data::Monitors(m) => m.len(),
            Data::Modes(m) => m.len(),
            Data::RefreshRates(r) => r.len(),
        };

        let Some(abs_index) = index.checked_abs() else {
            return Err(PyIndexError::new_err("Index out of bounds"));
        };
        let abs_index: usize = abs_index.try_into()?;
        let abs_index = abs_index.saturating_sub(1);

        // convert index to appropriate index
        let index = if index >= 0 {
            let len: isize = len.try_into()?;
            if index >= len {
                return Err(PyIndexError::new_err("Index out of bounds"));
            }

            // this is > 0, no signs are lost
            #[allow(clippy::cast_sign_loss)]
            {
                index as usize
            }
        } else if let Some(index) = (len.saturating_sub(1)).checked_sub(abs_index) {
            index
        } else {
            return Err(PyIndexError::new_err("Index out of bounds"));
        };

        Ok(index)
    }

    fn inner_mon(&self) -> &[Py<PyMonitor>] {
        match &self.0 {
            Data::Monitors(m) => m,
            _ => unreachable!(),
        }
    }

    fn inner_mon_mut(&mut self) -> &mut Vec<Py<PyMonitor>> {
        match &mut self.0 {
            Data::Monitors(m) => m,
            _ => unreachable!(),
        }
    }

    fn inner_mode(&self) -> &[Py<PyMode>] {
        match &self.0 {
            Data::Modes(m) => m,
            _ => unreachable!(),
        }
    }

    fn inner_rr(&self) -> &[Py<PyRefreshRate>] {
        match &self.0 {
            Data::RefreshRates(r) => r,
            _ => unreachable!(),
        }
    }
}

impl Display for PyContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Debug for PyContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::with_gil(|py| {
            write!(f, "[")?;

            match &self.0 {
                Data::Monitors(m) => {
                    let mut iter = m.iter().peekable();

                    while let Some(m) = iter.next() {
                        let m = &*m.borrow(py);

                        if iter.peek().is_some() {
                            write!(f, "{m:?}, ")?;
                        } else {
                            write!(f, "{m:?}")?;
                        }
                    }
                }

                Data::Modes(m) => {
                    let mut iter = m.iter().peekable();

                    while let Some(m) = iter.next() {
                        let m = &*m.borrow(py);

                        if iter.peek().is_some() {
                            write!(f, "{m:?}, ")?;
                        } else {
                            write!(f, "{m:?}")?;
                        }
                    }
                }

                Data::RefreshRates(r) => {
                    let mut iter = r.iter().peekable();

                    while let Some(rr) = iter.next() {
                        let rr = &*rr.borrow(py);

                        if iter.peek().is_some() {
                            write!(f, "{rr:?}, ")?;
                        } else {
                            write!(f, "{rr:?}")?;
                        }
                    }
                }
            }

            write!(f, "]")?;

            Ok(())
        })?;

        Ok(())
    }
}

#[derive(Debug)]
#[pyclass]
#[pyo3(name = "DriverClient")]
struct PyDriverClient {
    client: DriverClient,
    thread_registry: Arc<Mutex<Option<HANDLE>>>,
    #[pyo3(get, set)]
    monitors: Py<PyContainer>,
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

        let monitors = state_to_python(client.monitors(), py)?;

        let slf = Self {
            client,
            monitors,
            thread_registry: Arc::new(Mutex::new(None)),
        };

        Ok(slf)
    }

    /// Persist monitor configuration for user
    fn persist(&mut self, py: Python) -> PyResult<()> {
        self.validate(py)?;

        let state = python_to_state(&self.monitors, py);
        self.client.set_monitors(&state)?;

        self.client.persist()?;

        Ok(())
    }

    /// Manually refresh internal state with latest driver changes
    /// This will overwrite `monitors`
    fn refresh_state(&mut self, py: Python) -> PyResult<()> {
        self.client.refresh_state()?;

        self.monitors = state_to_python(self.client.monitors(), py)?;

        Ok(())
    }

    /// Send notification to driver of changes
    fn notify(&mut self, py: Python) -> PyResult<()> {
        self.validate(py)?;

        let state = python_to_state(&self.monitors, py);
        self.client.set_monitors(&state)?;

        self.client.notify()?;

        Ok(())
    }

    /// Get notified of other clients changing driver configuration
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
                        let state = state_to_python(&data, py);
                        let Ok(state) = state else {
                            println!("{}", state.unwrap_err());
                            return;
                        };

                        if let Err(e) = callback.call1(py, (state,)) {
                            println!("{e}");
                        }
                    });
                }
            },
        );
    }

    /// Validate the monitors
    /// Note: if data is stale and driver has a monitor id that already exists, this may erroneously return true,
    fn valid(&self, py: Python) -> bool {
        self.validate(py).is_ok()
    }

    /// Find id of monitor based on string
    fn find_id(&self, query: &str) -> Option<Id> {
        self.client.find_id(query).ok()
    }

    /// Find a monitor by Id
    fn find_monitor(&self, py: Python, query: Id) -> Option<Py<PyMonitor>> {
        self.monitors
            .borrow(py)
            .inner_mon()
            .iter()
            .find(|mon| mon.borrow(py).id == query)
            .cloned()
    }

    /// Find a monitor by name
    fn find_monitor_query(&self, py: Python, query: &str) -> Option<Py<PyMonitor>> {
        let num = query.parse::<Id>();

        self.monitors
            .borrow(py)
            .inner_mon()
            .iter()
            .find(|mon| {
                let mon = mon.borrow(py);
                mon.name.as_deref().is_some_and(|name| name == query)
                    || num.as_ref().is_ok_and(|&id| id == mon.id)
            })
            .cloned()
    }

    /// Get the closest available free ID. Note that if internal state is stale, this may result in a duplicate ID
    /// which the driver will ignore when you notify it of changes
    fn new_id(&mut self, py: Python, preferred_id: Option<Id>) -> Option<Id> {
        let state = python_to_state(&self.monitors, py);
        // by setting this, we can ensure it's up to date before trying to get the new id
        self.client.set_monitors(&state).ok()?;
        self.client.new_id(preferred_id).ok()
    }

    /// Remove monitors by id
    #[allow(clippy::needless_pass_by_value)]
    fn remove(&mut self, py: Python, ids: Vec<Id>) -> PyResult<()> {
        self.monitors
            .borrow_mut(py)
            .inner_mon_mut()
            .retain(|mon| !ids.contains(&mon.borrow(py).id));

        // keep internal state of client consistent
        let state = python_to_state(&self.monitors, py);
        self.client.set_monitors(&state)?;
        Ok(())
    }

    /// Enable monitors by id
    #[allow(clippy::needless_pass_by_value)]
    fn set_enabled(&mut self, py: Python, ids: Vec<Id>, enabled: bool) -> PyResult<()> {
        for mon in self.monitors.borrow(py).inner_mon() {
            let mut mon = mon.borrow_mut(py);
            if ids.contains(&mon.id) {
                mon.enabled = enabled;
            }
        }

        // keep internal state of client consistent
        let state = python_to_state(&self.monitors, py);
        self.client.set_monitors(&state)?;
        Ok(())
    }

    /// Enable monitors by id
    #[allow(clippy::needless_pass_by_value)]
    fn set_enabled_query(
        &mut self,
        py: Python,
        queries: Vec<String>,
        enabled: bool,
    ) -> PyResult<()> {
        let ids = queries
            .iter()
            .map(|query| query.parse::<Id>())
            .collect::<Vec<_>>();

        let contains_id = |id| {
            for pid in &ids {
                if pid.as_ref().is_ok_and(|&pid| pid == id) {
                    return true;
                }
            }

            false
        };

        for mon in self.monitors.borrow(py).inner_mon() {
            let mut mon = mon.borrow_mut(py);

            if contains_id(mon.id) {
                mon.enabled = enabled;
            }

            if let Some(name) = mon.name.as_ref() {
                if queries.contains(name) {
                    mon.enabled = enabled;
                }
            }
        }

        // keep internal state of client consistent
        let state = python_to_state(&self.monitors, py);
        self.client.set_monitors(&state)?;
        Ok(())
    }

    fn __repr__(&self, py: Python) -> String {
        format!(
            "DriverClient {{ monitors: {:?} }}",
            self.monitors.borrow(py)
        )
    }
}

impl PyDriverClient {
    fn validate(&self, py: Python) -> PyResult<()> {
        let monitor_iter = self.monitors.borrow(py);
        let monitor_iter = monitor_iter.inner_mon().iter().map(|mon| mon.borrow(py));

        for monitor in monitor_iter.clone() {
            let duplicate_id = monitor_iter.clone().any(|b| monitor.id == b.id);
            if duplicate_id {
                return Err(PyRuntimeError::new_err(format!(
                    "Found duplicate monitor id {}",
                    monitor.id
                )));
            }

            let mode_iter = monitor.modes.borrow(py);
            let mode_iter = mode_iter.inner_mode().iter().map(|mode| mode.borrow(py));
            for mode in mode_iter.clone() {
                let duplicate_mode = mode_iter
                    .clone()
                    .any(|m| mode.height == m.height && mode.width == m.width);

                if duplicate_mode {
                    return Err(PyRuntimeError::new_err(format!(
                        "Found duplicate mode {}x{} on monitor {}",
                        mode.width, mode.height, monitor.id
                    )));
                }

                let refresh_iter = mode.refresh_rates.borrow(py);
                let refresh_iter = refresh_iter.inner_rr().iter().map(|rr| rr.borrow(py));
                for rr in refresh_iter.clone() {
                    let duplicate_rr = refresh_iter.clone().any(|r| rr.0 == r.0);

                    if duplicate_rr {
                        return Err(PyRuntimeError::new_err(format!(
                            "Found duplicate refresh rate {} on mode {}x{} for monitor {}",
                            rr.0, mode.width, mode.height, monitor.id
                        )));
                    }
                }
            }
        }

        Ok(())
    }
}

impl Drop for PyDriverClient {
    fn drop(&mut self) {
        INIT.store(false, Ordering::Relaxed);
    }
}

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "Monitor")]
struct PyMonitor {
    #[pyo3(get, set)]
    id: Id,
    #[pyo3(get, set)]
    name: Option<String>,
    #[pyo3(get, set)]
    enabled: bool,
    #[pyo3(get, set)]
    modes: Py<PyContainer>,
}

#[pymethods]
impl PyMonitor {
    #[new]
    fn new(py: Python) -> PyResult<Self> {
        let inst = Self {
            id: 0,
            name: None,
            enabled: false,
            modes: Py::new(py, PyContainer(Data::Modes(Vec::new())))?,
        };

        Ok(inst)
    }

    /// Validate that Monitor settings are OK
    /// Note: Does not validate Id is ok. To do that, assign monitor to client and run client validate()
    fn valid(&self, py: Python) -> bool {
        let modes = self.modes.borrow(py);
        let modes = modes.inner_mode().iter().map(|mode| mode.borrow(py));

        for mode in modes.clone() {
            let dupes = modes
                .clone()
                .any(|m| m.width == mode.width && m.height == mode.height);

            if dupes {
                return false;
            }

            // check no conflicting modes and no conflicting refresh rates
            let rr = mode.refresh_rates.borrow(py);
            let rr_iter = rr.inner_rr().iter().map(|rr| rr.borrow(py));

            for rr in rr_iter.clone() {
                if rr_iter.clone().any(|r| r.0 == rr.0) {
                    return false;
                }
            }
        }

        true
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

impl std::fmt::Debug for PyMonitor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Python::with_gil(|py| {
            let PyMonitor {
                id,
                name,
                enabled,
                modes,
            } = self;

            f.debug_struct("Monitor")
                .field("id", &id)
                .field("name", &name)
                .field("enabled", &enabled)
                .field("modes", &modes.borrow(py))
                .finish()
        })
    }
}

#[derive(Clone)]
#[pyclass]
#[pyo3(name = "Mode")]
struct PyMode {
    #[pyo3(get, set)]
    width: Dimen,
    #[pyo3(get, set)]
    height: Dimen,
    #[pyo3(get, set)]
    refresh_rates: Py<PyContainer>,
}

impl Debug for PyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Python::with_gil(|py| {
            let PyMode {
                width,
                height,
                refresh_rates,
            } = self;

            f.debug_struct("Mode")
                .field("width", &width)
                .field("height", &height)
                .field("refresh_rates", &refresh_rates.borrow(py))
                .finish()
        })
    }
}

#[pymethods]
impl PyMode {
    #[new]
    fn new(py: Python) -> PyResult<Self> {
        let inst = Self {
            width: 0,
            height: 0,
            refresh_rates: Py::new(py, PyContainer(Data::RefreshRates(Vec::new())))?,
        };

        Ok(inst)
    }

    /// Checks whether this mode is valid as a mode in and of itself
    /// Note: If this is a duplicate mode in a list of modes, that would make this mode invalid
    ///       This does not check the valid status in a list, for that, use valid() on a monitor instance
    fn valid(&self, py: Python) -> bool {
        let rr = self.refresh_rates.borrow(py);
        let rr_iter = rr.inner_rr().iter().map(|rr| rr.borrow(py));

        for rr in rr_iter.clone() {
            if rr_iter.clone().any(|r| r.0 == rr.0) {
                return false;
            }
        }

        true
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

fn state_to_python(monitors: &[Monitor], py: Python) -> PyResult<Py<PyContainer>> {
    let mut py_state = Vec::new();

    for monitor in monitors {
        let mut modes = Vec::new();

        for mode in &monitor.modes {
            let py_refresh_rates = mode
                .refresh_rates
                .iter()
                .map(|rr| Py::new(py, PyRefreshRate(*rr)))
                .collect::<PyResult<Vec<_>>>()?;

            let container = Py::new(py, PyContainer(Data::RefreshRates(py_refresh_rates)))?;

            let mode = Py::new(
                py,
                PyMode {
                    width: mode.width,
                    height: mode.height,
                    refresh_rates: container,
                },
            )?;

            modes.push(mode);
        }

        let modes = Py::new(py, PyContainer(Data::Modes(modes)))?;

        let monitor = Py::new(
            py,
            PyMonitor {
                id: monitor.id,
                name: monitor.name.clone(),
                enabled: monitor.enabled,
                modes,
            },
        )?;

        py_state.push(monitor);
    }

    let py_state = Py::new(py, PyContainer(Data::Monitors(py_state)))?;

    Ok(py_state)
}

fn python_to_state(monitors: &Py<PyContainer>, py: Python) -> Vec<Monitor> {
    let mut state = Vec::new();

    let monitors = monitors.borrow(py);
    let monitors = monitors.inner_mon();

    for monitor in monitors.iter().map(|mon| mon.borrow(py)) {
        let mut modes = Vec::new();

        let py_modes = monitor.modes.borrow(py);
        let py_modes = py_modes.inner_mode();

        for mode in py_modes {
            let mode = mode.borrow(py);

            let refresh_rates = mode
                .refresh_rates
                .borrow(py)
                .inner_rr()
                .iter()
                .map(|rr| rr.borrow(py).0)
                .collect();

            modes.push(Mode {
                width: mode.width,
                height: mode.height,
                refresh_rates,
            });
        }

        state.push(Monitor {
            id: monitor.id,
            name: monitor.name.clone(),
            enabled: monitor.enabled,
            modes,
        });
    }

    state
}
