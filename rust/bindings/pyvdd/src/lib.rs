#![allow(unsafe_op_in_unsafe_fn, non_snake_case, non_local_definitions)]
//       ^                                       ^
//       |                                       |-this one seems triggered by pymethods macro??
//       |-this module triggers this lint unfortunately, so it must be set to allow

use driver_ipc::{
    ClientCommand, Dimen, DriverClient, EventCommand, Id, Mode, Monitor, RefreshRate,
};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "pyvdd")]
fn extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDriverClient>()?;
    m.add_class::<PyMonitor>()?;
    m.add_class::<PyMode>()?;

    Ok(())
}

#[derive(Debug)]
#[pyclass]
#[pyo3(name = "DriverClient")]
struct PyDriverClient {
    client: DriverClient,
    #[pyo3(get)]
    monitors: Vec<Py<PyMonitor>>,
}

#[pymethods]
impl PyDriverClient {
    #[new]
    fn new(py: Python) -> PyResult<Self> {
        let mut client = DriverClient::new()?;
        client.refresh_state()?;

        let slf = Self {
            monitors: state_to_python(client.monitors(), py)?,
            client,
        };

        Ok(slf)
    }

    #[setter]
    fn set_monitors(&mut self, py: Python, monitors: Vec<Py<PyMonitor>>) -> PyResult<()> {
        self.validate(py)?;
        let state = python_to_state(&monitors, py);
        self.client.set_monitors(&state)?;
        self.monitors = monitors;

        Ok(())
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
        self.client.set_receiver(move |command| {
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
        });
    }

    /// Validate the monitors
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
            .iter()
            .find(|mon| mon.borrow(py).id == query)
            .cloned()
    }

    /// Find a monitor by query
    fn find_monitor_query(&self, py: Python, query: &str) -> Option<Py<PyMonitor>> {
        let num = query.parse::<Id>();

        self.monitors
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
    fn new_id(&self, preferred_id: Option<Id>) -> Option<Id> {
        self.client.new_id(preferred_id).ok()
    }

    /// Remove monitors by id
    #[allow(clippy::needless_pass_by_value)]
    fn remove(&mut self, py: Python, ids: Vec<Id>) -> PyResult<()> {
        self.monitors
            .retain(|mon| !ids.contains(&mon.borrow(py).id));

        // keep internal state of client consistent
        let state = python_to_state(&self.monitors, py);
        self.client.set_monitors(&state)?;
        Ok(())
    }

    /// Enable monitors by id
    #[allow(clippy::needless_pass_by_value)]
    fn set_enabled(&mut self, py: Python, ids: Vec<Id>, enabled: bool) -> PyResult<()> {
        for mon in &self.monitors {
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

        for mon in &self.monitors {
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
}

impl PyDriverClient {
    fn validate(&self, py: Python) -> PyResult<()> {
        let mut monitor_iter = self.monitors.iter().map(|mon| mon.borrow(py));

        while let Some(monitor) = monitor_iter.next() {
            let duplicate_id = monitor_iter.clone().any(|b| monitor.id == b.id);
            if duplicate_id {
                return Err(PyRuntimeError::new_err(format!(
                    "Found duplicate monitor id {}",
                    monitor.id
                )));
            }

            let mut mode_iter = monitor.modes.iter();
            while let Some(mode) = mode_iter.next() {
                let duplicate_mode = mode_iter
                    .clone()
                    .any(|m| mode.height == m.height && mode.width == m.width);

                if duplicate_mode {
                    return Err(PyRuntimeError::new_err(format!(
                        "Found duplicate mode {}x{} on monitor {}",
                        mode.width, mode.height, monitor.id
                    )));
                }

                let mut refresh_iter = mode.refresh_rates.iter().copied();
                while let Some(rr) = refresh_iter.next() {
                    let duplicate_rr = refresh_iter.clone().any(|r| rr == r);
                    if duplicate_rr {
                        return Err(PyRuntimeError::new_err(format!(
                            "Found duplicate refresh rate {rr} on mode {}x{} for monitor {}",
                            mode.width, mode.height, monitor.id
                        )));
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
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
    modes: Vec<PyMode>,
}

#[pymethods]
impl PyMonitor {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Default)]
#[pyclass]
#[pyo3(name = "Mode")]
struct PyMode {
    #[pyo3(get, set)]
    width: Dimen,
    #[pyo3(get, set)]
    height: Dimen,
    #[pyo3(get, set)]
    refresh_rates: Vec<RefreshRate>,
}

#[pymethods]
impl PyMode {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

fn state_to_python(monitors: &[Monitor], py: Python) -> PyResult<Vec<Py<PyMonitor>>> {
    let mut py_state = Vec::new();

    for monitor in monitors {
        let mut modes = Vec::new();

        for mode in &monitor.modes {
            modes.push(PyMode {
                width: mode.width,
                height: mode.height,
                refresh_rates: mode.refresh_rates.clone(),
            });
        }

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

    Ok(py_state)
}

fn python_to_state(monitors: &[Py<PyMonitor>], py: Python) -> Vec<Monitor> {
    let mut state = Vec::new();

    for monitor in monitors.iter().map(|mon| mon.borrow(py)) {
        let mut modes = Vec::new();

        for mode in &monitor.modes {
            modes.push(Mode {
                width: mode.width,
                height: mode.height,
                refresh_rates: mode.refresh_rates.clone(),
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
