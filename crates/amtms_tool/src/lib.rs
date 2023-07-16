use bevy::app::{App, Plugin};

pub mod camera;
pub mod clip;
pub mod register;
pub mod api;
pub mod prelude;

pub struct AmtmsToolPlugin;

impl Plugin for AmtmsToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(camera::camera_controller::CameraControllerPlugin);
    }
}
pub fn sync_tool_status(){

}
