use bevy::app::{App, Plugin};
use crate::camera::prelude::CameraToolPlugin;
use crate::clip::prelude::ClipToolPlugin;

pub struct ToolPlugin;
impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ClipToolPlugin)
            .add_plugin(CameraToolPlugin)
        ;
    }
}