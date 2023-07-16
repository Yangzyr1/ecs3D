use bevy::asset::{Assets, Handle};
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::{Color, Commands, Component, Entity, Mesh, Query, ResMut, Transform, Without};
use bevy::render::mesh::PrimitiveTopology::LineList;

#[derive(Component, Clone)]
pub struct RenderableMesh(pub Mesh);
pub fn render_mesh(query: Query<(Entity, &mut RenderableMesh), (Without<Transform>)>,
                       mut commands: Commands,
                       mut meshes: ResMut<Assets<Mesh>>,
                       mut materials: ResMut<Assets<StandardMaterial>>){
    // let material = materials.add(Color::ORANGE.into());
    query.for_each(|(entity, mesh)| {
        let material = materials.add(Color::WHITE.into());
        let mesh_handle = meshes.add(mesh.0.clone());
        commands.entity(entity).insert(PbrBundle {
            mesh: mesh_handle,
            material,
            transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..Default::default()
        });
        if mesh.0.primitive_topology() == LineList { commands.entity(entity).insert(Wireframe); }
    });
}
#[derive(Component, Clone)]
pub struct RefreshableMesh(pub Mesh);
pub fn refresh_mesh(query: Query<(Entity, &mut RefreshableMesh, &mut Handle<Mesh>)>,
                    mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<StandardMaterial>>) {
    query.for_each(|(entity,mesh, renderable)| {
        let handler = meshes.add(mesh.0.clone());
        commands.entity(entity).insert(handler);
        commands.entity(entity).remove::<RefreshableMesh>();
        meshes.remove(renderable);
    });
}