use std::{env, fs};
use std::io::{BufRead, Error};
use std::path::{Path, PathBuf};
use bevy::prelude::{App, Plugin};
use clap::Parser;
/// 用于配置Python运行环境
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct PythonArgs {
    /// Python解释器地址,将会赋值给PYO3_PYTHON
    #[arg(long="py_env")]
    interpreter: Option<String>,

    /// Python包搜索路径,会赋值给PYTHON_PATH
    #[arg(long="py_pkg")]
    package: Option<String>,
}
pub struct PythonPlugin;
pub fn get_script_content(filename: &str) -> Result<String, Error> {
    //目前是获取项目路径
    let path = env::current_dir().unwrap();
    let full_path = path.join("scripts").join("python").join(filename);
    Ok(fs::read_to_string(full_path)?)
}
pub fn set_venv_path() {
    let mut args = PythonArgs::parse();
    if args.interpreter.is_none() {
        let path = env::current_dir().unwrap();
        let full_path = path.join("env");
        args.interpreter.insert(String::from(full_path.to_str().unwrap()));
    }
    if args.package.is_none() {
        let path = env::current_dir().unwrap();
        let full_path = path.join("env").join("Lib").join("site-packages");
        args.package.insert(String::from(full_path.to_str().unwrap()));
    }
    env::set_var("PYTHONHOME", args.interpreter.unwrap());
    env::set_var("PYTHONPATH", args.package.unwrap());
}
impl Plugin for PythonPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_venv_path);
    }
}
    