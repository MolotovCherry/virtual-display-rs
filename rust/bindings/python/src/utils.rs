use std::fmt::Display;

use pyo3::{exceptions::PyRuntimeError, prelude::PyErr};

pub trait IntoPyErr<T> {
    fn into_py_err(self) -> Result<T, PyErr>;
}

impl<T, E: Display> IntoPyErr<T> for Result<T, E> {
    fn into_py_err(self) -> Result<T, PyErr> {
        self.map_err(|err| PyRuntimeError::new_err(err.to_string()))
    }
}
