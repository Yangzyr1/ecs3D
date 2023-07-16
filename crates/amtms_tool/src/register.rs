use std::any::Any;
use std::mem::discriminant;
use std::sync::{Mutex};
use bevy::prelude::{Resource};
use once_cell::sync::Lazy;

pub enum Tools{
    Camera,
    Clip,
    Custom(String)
}
#[derive(Resource)]
pub struct ActiveTool(pub Tools);
pub static ACTIVE_TOOL_ARC: Lazy<Mutex<ActiveTool>> = Lazy::new(||{
    Mutex::new(ActiveTool(Tools::Camera))
});
pub fn check_mode(tool: Tools) -> bool{
    match tool {
        Tools::Camera => {
            let guard = ACTIVE_TOOL_ARC.lock().unwrap();
            discriminant(&guard.0) == discriminant(&Tools::Camera)
        },
        Tools::Clip => {
            let guard = ACTIVE_TOOL_ARC.lock().unwrap();
            discriminant(&guard.0) == discriminant(&Tools::Clip)
        },
        Tools::Custom(_) => {
            true
        }
    }
}
