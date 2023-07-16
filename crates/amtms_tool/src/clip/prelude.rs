use bevy::app::{App, Plugin};
use crate::clip::mesh_regenerator::ClipToolMeshPlugin;
use crate::clip::plane_generator::ClipToolPlanePlugin;
use crate::clip::ui::ClipToolUiPlugin;

pub struct ClipToolPlugin;
impl Plugin for ClipToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ClipToolUiPlugin)
            .add_plugin(ClipToolPlanePlugin)
            .add_plugin(ClipToolMeshPlugin);
    }
}