use bevy::prelude::{App, Plugin};
use crate::camera::camera_controller::CameraControllerPlugin;
use crate::camera::location_ui::{create_camera_position_ui, update_camera_position_ui};

pub struct CameraToolPlugin;
impl Plugin for CameraToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraControllerPlugin)
            .add_startup_system(create_camera_position_ui)
            .add_system(update_camera_position_ui);
    }
}