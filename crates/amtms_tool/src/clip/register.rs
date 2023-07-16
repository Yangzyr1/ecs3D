use std::collections::HashMap;
use bevy::math::Vec3;
use bevy::prelude::{Component, Mesh, Resource};
use bevy::render::primitives::Plane;

#[derive(Resource)]
pub struct ClipPlain {
    // 平面
    pub plane: Plane,
    // XZ平面上指示逆时针方向的向量
    // 该向量指向的方向为逆时针底边
    // 逆时针方向是法线通过右手定则确定的方向
    pub basis: Vec3
}
#[derive(Component, Clone)]
pub struct ClipableMesh {
    pub vertices: Vec<[f32;3]>,
    pub indices: Vec<u32>,
    pub color: Vec<[f32;4]>,
}
pub enum ClipMode {
    PointLine,
    DragLine,
    PointPlain,
    DragPlain,
}
/// 剖面本身
#[derive(Default, Clone, Component, Debug)]
pub struct ClipFace {
    pub points: Vec<[f32;3]>,
    // pub mapping: HashMap<usize, usize>,
    pub indices: Vec<u32>,
    pub color: Vec<[f32;4]>,
}
