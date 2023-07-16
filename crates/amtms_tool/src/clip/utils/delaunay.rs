use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::Add;
use std::sync::{Arc, RwLock};
use bevy::log::info;
use bevy::math::Vec3;
use bevy::prelude::Quat;
use delaunator::{Point, triangulate};


use crate::clip::register::ClipFace;
pub struct CombinedPoints32(pub Vec<[f32;3]>, pub Vec<[f32;4]>);
fn get_normal_quat(normal: Vec3) -> Quat {
    let projection_xz = Vec3::new(normal.x, 0.0, normal.z);
    let x_xz = (projection_xz.x.powi(2) + projection_xz.z.powi(2)).sqrt();
    let y_xz = projection_xz.z.abs();
    let mut theta_xz = y_xz.atan2(x_xz) * 180.0 / PI;
    if projection_xz.x <= 0.0 {
        theta_xz = 2.0 * PI - theta_xz;
    }
    let projection_yz = Vec3::new(0.0, normal.y, normal.z);
    let x_yz = (projection_yz.y.powi(2) + projection_yz.z.powi(2)).sqrt();
    let y_yz = projection_yz.z.abs();
    let mut theta_yz = y_yz.atan2(x_yz) * 180.0 / PI;
    if projection_yz.y <= 0.0 {
        theta_yz = 2.0 * PI - theta_yz;
    }
    info!("Vector:{:?},XZ{:?},YZ{:?}",normal, theta_xz,theta_yz);
    let q_y = Quat::from_rotation_y(theta_xz);
    let q_x = Quat::from_rotation_x(theta_yz);
    return q_x.mul_quat(q_y);
}
pub fn combine_points(points: Vec<[f32;3]>, color: Vec<[f32;4]>, threshold: u32) -> CombinedPoints32{
    let mut temp_hashmap :HashMap<String, usize> = HashMap::new();
    let mut result = Vec::new();
    let mut result_color = Vec::new();
    for index in 0..points.len() {
        let point = points[index];
        let mut point_i32: Vec<i32> = Vec::new();
        for index in point {
            point_i32.push((index * threshold as f32).round() as i32);
        }
        let point_hash = point_i32[0].to_string().add(point_i32[1].to_string().as_str()).add(point_i32[2].to_string().as_str());
        if !temp_hashmap.contains_key(point_hash.as_str()) {
            temp_hashmap.insert(point_hash.clone(), temp_hashmap.len());
            result.push(point);
            result_color.push(color[index]);
        }
    }
    return CombinedPoints32(result,result_color);
}
pub fn prepare_point_set_32(points: Vec<[f32;3]>, threshold: u32) -> ClipFace {
    let mut temp_hashmap :HashMap<String, usize> = HashMap::new();
    let mut find_set = ClipFace::default();
    for index in 0..points.len() {
        let point = points[index];
        let mut point_i32: Vec<i32> = Vec::new();
        for index in point {
            point_i32.push((index * threshold as f32).round() as i32);
        }
        let point_hash = point_i32[0].to_string().add(point_i32[1].to_string().as_str()).add(point_i32[2].to_string().as_str());
        if !temp_hashmap.contains_key(point_hash.as_str()) {
            temp_hashmap.insert(point_hash.clone(), temp_hashmap.len());
            find_set.points.push(point);
        }
        //TODO Mapping
        // find_set.mapping.insert(index, *temp_hashmap.get(point_hash.as_str()).unwrap());
    }
    return find_set;
}
pub fn projection(points: Vec<[f32;3]>, normal: Vec3) -> Vec<[f32;2]> {
    let quat = get_normal_quat(normal);
    let mut result: Vec<[f32;2]> = Vec::new();
    for point in points {
        let point_mut = quat.mul_vec3(Vec3::from_array(point));
        // info!("{:?}", point_mut);
        result.push([point_mut.x, point_mut.y]);
    }
    return result;
}
pub fn delaunay(points: Vec<[f32;2]>) -> Vec<usize> {
    let mut delaunay_point = Vec::new();
    for point in points {
        delaunay_point.push(Point{x: point[0] as f64, y: point[1] as f64 });
    }
    let result = triangulate(&delaunay_point);
    return result.triangles;
}
/// 生成补面
pub fn generate_clip_face(points: Vec<[f32;3]>, color:Vec<[f32;4]>, normal: Vec3) -> ClipFace{
    let combined_points = combine_points(points, color, 1000);
    let mut clip_face = ClipFace::default();
    clip_face.points = combined_points.0.clone();
    clip_face.color = combined_points.1;
    clip_face.indices = delaunay(projection(combined_points.0, normal)).iter().map(|&x| x as u32).collect();
    return clip_face;
}