use std::sync::Arc;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::time::Duration;
use tonlib::{Config, TonlibClient};

#[pyclass]
struct PyTonClient {
    rt: Option<tokio::runtime::Runtime>,
    client: Arc<TonlibClient>,
}

#[pymethods]
impl PyTonClient {
    #[new]
    fn new(address: String, key: String) -> PyResult<Self> {
        let server_address = address.parse().map_err(PyValueError::new_err)?;
        let last_block_threshold = Duration::from_secs(1);

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let client = rt
            .block_on(TonlibClient::new(&Config {
                server_address,
                server_key: key,
                last_block_threshold,
            }))
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(Self {
            rt: Some(rt),
            client: Arc::new(client),
        })
    }

    fn send_message(mut self_: PyRefMut<Self>, bytes: &PyBytes) -> PyResult<()> {
        let mut rt = self_.rt.take();

        let result = match &mut rt {
            Some(rt) => rt
                .block_on(async { self_.client.send_message(bytes.as_bytes().to_vec()).await })
                .map_err(|e| PyValueError::new_err(e.to_string())),
            None => unreachable!(),
        };

        self_.rt = rt;

        result
    }
}

#[pymodule]
fn libpyton(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTonClient>()?;
    Ok(())
}
