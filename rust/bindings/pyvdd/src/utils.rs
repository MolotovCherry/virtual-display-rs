use pyo3::{exceptions::PyRuntimeError, prelude::PyErr};

pub trait IntoPyErr<T> {
    fn into_py_err(self) -> Result<T, PyErr>;
}

impl<T> IntoPyErr<T> for driver_ipc::Result<T> {
    fn into_py_err(self) -> Result<T, PyErr> {
        self.map_err(|err| PyRuntimeError::new_err(err.to_string()))
    }
}
