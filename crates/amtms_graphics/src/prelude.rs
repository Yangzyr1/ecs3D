use bevy::app::{App, Plugin};
use crate::render::prelude::{refresh_mesh, render_mesh};

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_mesh)
            .add_system(refresh_mesh);
    }
}