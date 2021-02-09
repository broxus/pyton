use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ton_block::Deserializable;

#[pyclass]
struct PyContractInfo {
    #[pyo3(get)]
    code_hash: String,
    #[pyo3(get)]
    data_hash: String,
}

#[pyfunction]
fn get_contract_info(tvc: &[u8]) -> PyResult<PyContractInfo> {
    let data = ton_types::deserialize_tree_of_cells(&mut std::io::Cursor::new(tvc))
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let account = ton_block::StateInit::construct_from_cell(data)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let code_hash = account
        .code
        .map(|code| code.repr_hash())
        .unwrap_or_default()
        .to_hex_string();
    let data_hash = account
        .data
        .map(|code| code.repr_hash())
        .unwrap_or_default()
        .to_hex_string();

    Ok(PyContractInfo {
        code_hash,
        data_hash,
    })
}

#[pymodule]
fn pyton(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyContractInfo>()?;
    m.add_function(pyo3::wrap_pyfunction!(get_contract_info, m)?)?;

    Ok(())
}
