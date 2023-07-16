use std::ops::Add;
use bevy::prelude::{AssetServer, Color, Commands, Component, default, PositionType, Query, Res, Style, Text, TextAlignment, TextBundle, TextStyle, Transform, UiRect, Val, With};
use crate::camera::camera_controller::CameraController;

#[derive(Component)]
pub struct CameraPositionUi;

pub fn create_camera_position_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "摄像机坐标信息",
            TextStyle {
                font: asset_server.load("fonts/yysfont.ttf"),
                font_size: 25.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        CameraPositionUi,
    ));
}
pub fn update_camera_position_ui(camera_query: Query<&Transform, With<CameraController>>,
                                 mut ui_query: Query<&mut Text, With<CameraPositionUi>>) {
    // ui_query.for_each(mut |text|{
    //     camera_query.for_each(|position|{
    //         text.sections[0].value = position.translation.to_string();
    //     });
    // })
    for mut text in &mut ui_query {
        let mut position_str = String::new() ;
        let mut rotation_str = String::new() ;
        camera_query.for_each(|position|{
            position_str = format!("摄像机坐标:X:{:.4},Y:{:.4},Z:{:.4}", position.translation.x, position.translation.y, position.translation.z);
            rotation_str = format!("朝向方向:X:{:.4},Y:{:.4},Z:{:.4},W:{:.4}", position.rotation.x, position.rotation.y, position.rotation.z,position.rotation.w);
        });
        text.sections[0].value = position_str.add("\n").add(rotation_str.as_str());


    }
}