use std::collections::HashMap;
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::{App, Assets, Color, Commands, Component, Resource, default, Entity, Mesh, PbrBundle, Plugin, Query, ResMut, shape, StandardMaterial, Transform, Vec3, With, Without};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::mesh::PrimitiveTopology::{LineList, TriangleList};
use amtms_graphics::render::prelude::RenderableMesh;
use amtms_tool::clip::register::ClipableMesh;
use crate::python_adapter::generate_gtp;
use crate::register::gtp::GTP_STATE;

#[derive(Resource)]
pub struct DrillTerrainDataPath(Vec<String>);
#[derive(Component, Clone, Debug)]
pub struct DrillPoint{
    point_id: i64,
    drill: i64,
    terrain: i64,
    x: f32,
    y: f32,
    z: f32,
    red: f32,
    green: f32,
    blue: f32
}
#[derive(Component,Clone, Debug)]
pub struct GTP{
    points: Vec<DrillPoint>,
    stone_type: String
}
#[derive(Component,Clone, Debug)]
pub struct ColorDrillPoint{
    x: f64,
    y: f64,
    z: f64,
    color: String
}
///用于生成GTP体的系统,不直接
pub fn create_gtp_system(mut commands: Commands){
    let mut gtp_resource = GTP_STATE.lock().unwrap();
    if !gtp_resource.refresh {
        return;
    }
    gtp_resource.refresh = false;
    let csv_path = gtp_resource.csv_path.as_str();
    let mut hm = HashMap::new();
    hm.insert(String::from("csv_path"), String::from(csv_path));
    println!("csv:{}", csv_path);
    let data: Vec<GTP> = generate_gtp(csv_path).expect("TODO: panic message");
    let mut total_vertices = Vec::new();
    let mut total_colors = Vec::new();
    let mut total_tri_indices = Vec::new();
    let mut total_line_indices = Vec::new();
    for gtp in data {
        let vertex_offset = total_vertices.len() as u32;
        for point in gtp.clone().points {
            total_vertices.push([point.x, point.y, point.z]);
            total_colors.push([point.red, point.green, point.blue, 1.0]);
        }
        let tri_indices = vec![
            // 底部三角形
            0, 1, 2,
            // 侧面三角形
            0, 3, 1,
            1, 3, 4,
            1, 4, 2,
            2, 4, 5,
            2, 5, 0,
            0, 5, 3,
            // 顶部三角形
            5, 4, 3,
        ];
        let line_indices = vec![
            // 三个侧面
            0, 3, 3, 4, 4, 1, 1, 0,
            0, 2, 2, 5, 5, 3, 3, 0,
            2, 1, 1, 4, 4, 5, 5, 2,
        ];
        for tri_index in tri_indices {
            total_tri_indices.push(tri_index + vertex_offset);
        }
        for line_index in line_indices {
            total_line_indices.push(line_index + vertex_offset);
        }
        commands.spawn(gtp);
    }
    // 创建线框和三角形的网格
    let mut line_mesh = Mesh::new(LineList);
    let mut tri_mesh = Mesh::new(TriangleList);
    line_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, total_vertices.clone());
    line_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, total_colors.clone());
    line_mesh.set_indices(Some(Indices::U32(total_line_indices.clone())));
    tri_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, total_vertices.clone());
    tri_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, total_colors.clone());
    tri_mesh.set_indices(Some(Indices::U32(total_tri_indices.clone())));
    // commands.spawn(RenderableMesh(line_mesh.clone())).insert(
    //     ClipableMesh{
    //         vertices: total_vertices.clone(),
    //         indices: total_line_indices.clone(),
    //         color: total_colors.clone()
    //     });
    commands.spawn(RenderableMesh(tri_mesh.clone())).insert(
        ClipableMesh{
            vertices: total_vertices.clone(),
            indices: total_tri_indices.clone(),
            color: total_colors.clone()
        });
}
pub struct GTPPlugin;
impl Plugin for GTPPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_gtp_system)
            .add_plugin(WireframePlugin);
    }
}