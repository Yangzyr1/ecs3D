use std::collections::HashMap;
use std::{env, fs};
use pyo3::prelude::PyModule;
use pyo3::{PyResult, Python};
use pyo3::types::IntoPyDict;
use pyo3::FromPyObject;
use crate::python_env::get_script_content;

#[derive(FromPyObject, Clone, Debug)]
pub struct DrillPoint{
    point_id: i64,
    drill: i64,
    terrain: i64,
    x: f32,
    y: f32,
    z: f32,
    red: f32,
    green: f32,
    blue: f32
}
#[derive(FromPyObject, Clone, Debug)]
pub struct GTP{
    points: Vec<DrillPoint>,
    stone_type: String
}
pub fn py_load_drill_point(data_path: &str) -> PyResult<Vec<DrillPoint>> {
    pyo3::prepare_freethreaded_python();
    let script_content = get_script_content("geo_drill.py")?;
    let mut kwargs: HashMap<String,String> = HashMap::new();
    kwargs.insert(String::from("csv_path"), String::from(data_path));
    Python::with_gil(|py| {
        let kwargs_py = kwargs.into_py_dict(py);
        let result: Vec<DrillPoint> = PyModule::from_code(
            py,
            &script_content,
            "",
            ""
        )?
            .getattr("load_drill_point")?
            .call((), Some(kwargs_py))?
            .extract()?;
        Ok(result)
    })
}
pub fn py_generate_gtp(data_path: &str) -> PyResult<Vec<GTP>> {
    let script_content = get_script_content("geo_drill.py")?;
    let mut kwargs: HashMap<String,String> = HashMap::new();
    kwargs.insert(String::from("csv_path"), String::from(data_path));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let kwargs_py = kwargs.into_py_dict(py);
        let result: Vec<GTP> = PyModule::from_code(
            py,
            &script_content,
            "",
            ""
        )?
            .getattr("generate_gtp")?
            .call((), Some(kwargs_py))?
            .extract()?;
        Ok(result)
    })
}
pub fn py_test() -> PyResult<()> {
    let script_content = get_script_content("nmsl.py")?;
    let mut kwargs: HashMap<String,String> = HashMap::new();
    pyo3::prepare_freethreaded_python();
    println!("PYO3_PYTHON: {:?}", env::var("path"));
    Python::with_gil(|py| {
        let kwargs_py = kwargs.into_py_dict(py);
        let result: Vec<String> = PyModule::from_code(
            py,
            &script_content,
            "",
            ""
        )?
            .getattr("nm")?
            .call((), Some(kwargs_py))?
            .extract()?;
        println!("wocao,{:?}", result);
        Ok(())
    })
}