use bevy::app::{App, Plugin};
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::{Commands, Entity, info, IntoSystemConfig, Mesh, Query, ResMut, resource_exists, Without};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use amtms_graphics::render::prelude::RenderableMesh;
use crate::clip::register::{ClipFace, ClipPlain};

use crate::register::check_mode;
use crate::register::Tools::Clip;

fn create_clip_face(query: Query<(Entity, &mut ClipFace), Without<RenderableMesh>>,
                    plane: ResMut<ClipPlain>,
                    mut commands: Commands){
    query.for_each(|(entity, mut face)|{
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, face.points.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, face.color.clone());
        mesh.set_indices(Some(Indices::U32(face.indices.clone())));
        commands.spawn(RenderableMesh(mesh));
    })
}
pub struct ClipToolFacePlugin;
impl Plugin for ClipToolFacePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_clip_face.run_if(resource_exists::<ClipPlain>()).run_if(||{
            check_mode(Clip)
        }));
    }
}