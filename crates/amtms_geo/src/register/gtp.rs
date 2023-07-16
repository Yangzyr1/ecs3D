use std::env;
use std::sync::{Mutex};
use once_cell::sync::Lazy;

pub static GTP_STATE: Lazy<Mutex<GtpState>> = Lazy::new(||{
    let path = env::current_dir().unwrap();
    let full_path = path.join("data").join("geo").join("gtp");
    let state = GtpState{
        refresh : true,
        csv_path: String::from(full_path.to_str().unwrap())
    };
    Mutex::new(state)
});
#[derive(Clone, Debug)]
pub struct GtpState{
    pub refresh: bool,
    pub csv_path: String
}