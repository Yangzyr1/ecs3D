use crate::register::{ACTIVE_TOOL_ARC, Tools};

pub fn change_tool_mode(mode: String) {
    let mut state = ACTIVE_TOOL_ARC.lock().unwrap();
    if mode == String::from("Camera") {
        state.0 = Tools::Camera;
    }
    if mode == String::from("Clip") {
        state.0 = Tools::Clip;
    }
}