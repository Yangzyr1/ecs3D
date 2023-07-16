use bevy::log::info;
use bevy::math::{Ray, Vec3, Vec4};
use bevy::prelude::{App, Plugin, Component, With, Query, Commands, MouseButton, GlobalTransform, Local, Res, Input, Window, Camera, IntoSystemConfig};
use bevy::render::primitives::Plane;
use crate::camera::camera_controller::CameraController;
use crate::clip::register::ClipPlain;
use crate::register::check_mode;
use crate::register::Tools::Clip;


#[derive(Component)]
struct ClipRay(Ray);
struct ClippedFace{
    vertices: Vec<Vec4>,
}

fn create_clip_plane(mut commands: Commands,
                    query: Query<(&Camera, &GlobalTransform), With<CameraController>>,
                    mouse_button_input: Res<Input<MouseButton>>,
                    mut ray: Local<Option<Ray>>,
                    windows: Query<&mut Window>) {
    // 按下鼠标左键时发射第一条射线
    if mouse_button_input.just_pressed(MouseButton::Left) {
        *ray = query.single().0.viewport_to_world(query.single().1,
                                                  windows.single().cursor_position().unwrap())
    }
    // 松开鼠标左键时发送第二条射线并生成平面
    if mouse_button_input.just_released(MouseButton::Left) {
        let ray1 = ray.unwrap();
        let ray2 = query.single().0.viewport_to_world(query.single().1,
                                                  windows.single().cursor_position().unwrap()).unwrap();
        let point = ray1.origin;
        let direction2 = ray2.origin - ray1.origin;
        let normal = ray1.direction.cross(direction2).normalize();
        let distance = point.dot(normal);
        let plane_vec4 = Vec4::new(normal.x,normal.y,normal.z,distance);
        let plane = Plane::new(plane_vec4);
        info!("切割平面: {:?}", plane);
        commands.insert_resource(ClipPlain {
            plane,
            basis: Vec3::new(ray1.direction.x, 0.0, ray1.direction.z)
        });
    }
}

pub struct ClipToolPlanePlugin;
impl Plugin for ClipToolPlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_clip_plane.run_if(||{
            check_mode(Clip)
        }));
    }
}