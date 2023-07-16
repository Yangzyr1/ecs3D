use std::{mem};
use std::error::Error;
use amtms_python::geo::{py_generate_gtp, py_load_drill_point};
use crate::gtp::{DrillPoint, GTP};

///读取单纯的点数据
pub fn load_drill_point(data_path: &str) -> Result<Vec<DrillPoint>, Box<dyn Error>> {
    let result_origin = py_load_drill_point(data_path)?;
    let mut result_transfer: Vec<DrillPoint> = Vec::new();
    for item in result_origin {
        result_transfer.push(unsafe {mem::transmute(item)});
    }
    Ok(result_transfer)
}
pub fn generate_gtp(data_path: &str) -> Result<Vec<GTP>, Box<dyn Error>> {
    let result_origin = py_generate_gtp(data_path)?;
    let mut result_transfer: Vec<GTP> = Vec::new();
    for item in result_origin {
        result_transfer.push(unsafe {mem::transmute(item)});
    }
    Ok(result_transfer)
}