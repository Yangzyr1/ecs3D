use bevy::app::{App, Plugin};
use bevy::math::{Ray, Vec3, Vec3A};
use bevy::prelude::{Commands, Component, Entity, IntoSystemConfig, IntoSystemSetConfig, Mesh, Query, ResMut, resource_exists, Without, World};
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology::TriangleList;
use crate::clip::register::{ClipableMesh, ClipFace, ClipPlain};
use crate::register::check_mode;
use crate::register::Tools::Clip;
use amtms_graphics::render::prelude::RenderableMesh;
use crate::clip::clip_plane_generator::ClipToolFacePlugin;
use crate::clip::utils::delaunay::{combine_points, generate_clip_face};


#[derive(Component, Clone)]
pub struct ClipIndices{
    pub direction: Vec<f32>,
    pub same: Vec<(u32,u32,u32)>,
    pub opposite: Vec<(u32,u32,u32)>,
    pub impact: Vec<(u32,u32,u32)>,
}
#[derive(Component, Clone)]
pub struct ClipIntersection{
    pub basis: Vec3,
    pub point: Vec<[f32;3]>,
    pub color: Vec<[f32;4]>,
    // 同向的索引，包含切割面的侧面三角形索引
    pub same: Vec<(u32,u32,u32)>,
    // 异向的索引，同上
    pub opposite: Vec<(u32,u32,u32)>,
    // 切割面的线索引，用于修复切割面
    pub intersection: Vec<(u32,u32)>,
}

///判别方向并将原先的网格分成2部分，返回点与索引
fn split_direction(mesh: Query<(Entity, &ClipableMesh), Without<ClipIndices>>,
                   plane: ResMut<ClipPlain>,
                   mut commands: Commands) {
    mesh.for_each(|(entity, clipable_mesh)|{
        //这是平面上的点，三角形的顶点减去它得到的向量和平面法向量点积可区分正负
        let plane_point = plane.plane.normal() * plane.plane.d();
        // 存储点积法线结果
        let mut direction_vec: Vec<f32> = Vec::new();
        // 存储法线同向三角形的索引
        let mut same_indices: Vec<(u32,u32,u32)> = Vec::new();
        // 存储法线异向三角形的索引
        let mut opposite_indices: Vec<(u32,u32,u32)> = Vec::new();
        // 存储碰撞三角形的索引
        let mut impact_indices: Vec<(u32,u32,u32)> = Vec::new();
        // 如果三角形面片的三个顶点同号，则无碰撞，直接分类即可
        // 如果三角形面片的顶点异号，则有碰撞，需要进行切割
        let vertices = clipable_mesh.vertices.clone();
        let indices = clipable_mesh.indices.clone();
        for vertex in vertices {
            direction_vec.push(plane.plane.normal().dot(Vec3A::from_array(vertex) - plane_point));
        }
        for index in (0..indices.len()).step_by(3) {
            if direction_vec[indices[index] as usize].is_sign_positive() && direction_vec[indices[index+1] as usize].is_sign_positive() && direction_vec[indices[index+2] as usize].is_sign_positive() {
                // 与法线正向，为原来的网格
                same_indices.push((indices[index], indices[index+1], indices[index+2]));
            }else if direction_vec[indices[index] as usize].is_sign_negative() && direction_vec[indices[index+1] as usize].is_sign_negative() && direction_vec[indices[index+2] as usize].is_sign_negative() {
                // 与法线异向，为新建网格
                opposite_indices.push((indices[index], indices[index+1], indices[index+2]));
            }else {
                // info!("{:?},{:?},{:?}",indices[index],indices[index+1], indices[index+2]);
                // 三角形顶点不同向，为碰撞情况
                impact_indices.push((indices[index], indices[index+1], indices[index+2]));
            }
        }
        commands.entity(entity).insert(ClipIndices{direction: direction_vec, same: same_indices, opposite: opposite_indices, impact: impact_indices});
    });
}
/// 根据分类结果生成新的网格信息
/// 新生成的切割面点的点数据直接加入ClipableMesh，索引直接形成三角形加入两类
/// 但是此时的顶点没有合并同类项
fn calculate_intersection(query: Query<(Entity, &ClipIndices, &mut ClipableMesh), Without<ClipIntersection>>,
                 plane: ResMut<ClipPlain>,
                 mut commands: Commands) {
    query.for_each(|(entity, indices, mesh)|{
        let plane_point = plane.plane.normal() * plane.plane.d();
        let mut clip_face_points = ClipFace::default();
        let mut points = mesh.clone();
        let mut indices_clone = indices.clone();
        let mut intersection = Vec::new();
        for triangle in indices.impact.clone() {
            // 根据方向乘积得出孤立点，相同方向乘积必大于零，给p0
            // 这里的顺序必须保持索引的排列顺序，因为要保持逆时针特性，即必须保持012的顺序
            let (p0, p1, p2) =
                if indices.direction[triangle.1 as usize] * indices.direction[triangle.2 as usize] > 0.0 { triangle }
            else if indices.direction[triangle.0 as usize] * indices.direction[triangle.2 as usize] > 0.0 { (triangle.1, triangle.2, triangle.0) }
            else { (triangle.2, triangle.0, triangle.1) };

            let p0_p1 = Vec3::from_array( mesh.vertices[p1 as usize])-Vec3::from_array(mesh.vertices[p0 as usize]);
            let p0_p2 = Vec3::from_array( mesh.vertices[p2 as usize])-Vec3::from_array(mesh.vertices[p0 as usize]);
            let c1_ray = Ray{direction: p0_p1, origin: Vec3::from_array(mesh.vertices[p0 as usize])};
            let c2_ray = Ray{direction: p0_p2, origin: Vec3::from_array(mesh.vertices[p0 as usize])};
            let c1_d = c1_ray.intersect_plane(Vec3::from(plane_point), Vec3::from(plane.plane.normal()));
            let c2_d = c2_ray.intersect_plane(Vec3::from(plane_point), Vec3::from(plane.plane.normal()));

            let c1 = c1_ray.get_point(c1_d.unwrap());
            let c2 = c2_ray.get_point(c2_d.unwrap());
            let p0_color = points.color[p0 as usize];
            let p1_color = points.color[p1 as usize];
            let p2_color = points.color[p2 as usize];
            let c1_color = [(p0_color[0]+p1_color[0])/2.0,(p0_color[1]+p1_color[1])/2.0,(p0_color[2]+p1_color[2])/2.0,(p0_color[3]+p1_color[3])/2.0];
            let c2_color = [(p0_color[0]+p2_color[0])/2.0,(p0_color[1]+p2_color[1])/2.0,(p0_color[2]+p2_color[2])/2.0,(p0_color[3]+p2_color[3])/2.0];
            points.color.push(c1_color);
            points.color.push(c2_color);
            points.vertices.push([c1.x,c1.y,c1.z]);
            points.vertices.push([c2.x,c2.y,c2.z]);
            clip_face_points.points.push(c1.to_array());
            clip_face_points.points.push(c2.to_array());
            clip_face_points.color.push(c1_color);
            clip_face_points.color.push(c2_color);
            let c1_loc = (points.vertices.len() - 2) as u32;
            let c2_loc = (points.vertices.len() - 1) as u32;
            if indices.direction[p0 as usize] > 0.0 {
                //孤立点与平面法向量同向
                indices_clone.same.push((p0, c1_loc, c2_loc));
                indices_clone.opposite.push((c1_loc, p1, p2));
                indices_clone.opposite.push((c2_loc, c1_loc, p2));
                intersection.push((c2_loc, c1_loc));
            } else {
                //孤立点与平面法向量反向
                indices_clone.opposite.push((p0, c1_loc, c2_loc));
                indices_clone.same.push((c1_loc, p1, p2));
                indices_clone.same.push((c2_loc, c1_loc, p2));
                intersection.push((c1_loc, c2_loc));
            }
        }
        clip_face_points = generate_clip_face(clip_face_points.points, clip_face_points.color, Vec3::from(plane.plane.normal()));
        commands.entity(entity).insert(ClipIntersection{
            point: points.vertices,
            color: points.color,
            same: indices_clone.same,
            opposite: indices_clone.opposite,
            intersection,
            basis: plane.basis
        });
        commands.spawn(clip_face_points);
    });
}
///修补新网格中的切割面
fn fix_clip_face(query: Query<(Entity, &ClipIntersection)>,
                 plane: ResMut<ClipPlain>,
                 mut commands: Commands) {
    query.for_each(|(entity, intersection)|{
        let mut tri_mesh = Mesh::new(TriangleList);
        let mut same: Vec<u32> = intersection.same.clone()
            .iter()
            .flat_map(|&(x, y, z)| vec![x, y, z])
            .collect();
        let mut inter: Vec<u32> = intersection.intersection.clone()
            .iter()
            .flat_map(|&(x, y)| vec![x, y])
            .collect();
        // same.extend(regenerate_clip_triangle(intersection.clone().point, inter, 1000, intersection.basis));
        tri_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, intersection.clone().point);
        tri_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, intersection.clone().color);
        tri_mesh.set_indices(Some(Indices::U32(same.clone())));
        commands.spawn(RenderableMesh(tri_mesh.clone()));//.insert(ClipableMesh{
        //     vertices: intersection.clone().point,
        //     indices: same.clone(),
        //     color: intersection.clone().color,
        // })
        // .insert(Wireframe);


        // let mut clip_face = ClipFace::default();
        // let mut indices: Vec<u32> = Vec::new();
        // for index in generate_clip_indices(intersection.clone(),Vec3::from(plane.plane.normal()) ) {
        //     indices.push(index as u32);
        // }
        // clip_face.indices = indices;
        // clip_face.points = combine_points(intersection.clone().point, 1000);
        // clip_face.color = ;
        // commands.spawn(clip_face);


        commands.entity(entity).despawn();

        // let mut tri_mesh_2 = Mesh::new(TriangleList);
        // let same: Vec<u32> = intersection.same.clone()
        //     .iter()
        //     .flat_map(|&(x, y, z)| vec![x, y, z])
        //     .collect();
        // tri_mesh_2.insert_attribute(Mesh::ATTRIBUTE_POSITION, intersection.clone().point);
        // tri_mesh_2.insert_attribute(Mesh::ATTRIBUTE_COLOR, intersection.clone().color);
        // tri_mesh_2.set_indices(Some(Indices::U32(same)));
        // commands.spawn(RenderableMesh(tri_mesh_2.clone())).insert(Wireframe);
    })
}
fn generate_clip_plane(){

}
pub struct ClipToolMeshPlugin;
impl Plugin for ClipToolMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(split_direction.run_if(resource_exists::<ClipPlain>()).run_if(||{
            check_mode(Clip)
        })).add_system(calculate_intersection.run_if(resource_exists::<ClipPlain>()).run_if(||{
            check_mode(Clip)
        })).add_system(fix_clip_face.run_if(resource_exists::<ClipPlain>()).run_if(||{
            check_mode(Clip)
        })).add_plugin(ClipToolFacePlugin);
    }
}
fn plane_existed(world: &mut World) -> bool {
    world.contains_resource::<ClipPlain>()
}
