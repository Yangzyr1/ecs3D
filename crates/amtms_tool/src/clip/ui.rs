use std::ops::Add;
use bevy::app::{App, Plugin};
use bevy::prelude::{AssetServer, Color, Commands, Component, default, Entity, IntoSystemConfig, PositionType, Query, Res, Style, Text, TextAlignment, TextBundle, TextStyle, Transform, UiRect, Val, With};
use crate::camera::camera_controller::CameraController;
use crate::register::check_mode;
use crate::register::Tools::Clip;

#[derive(Component)]
pub struct ClipModeUi;

pub fn create_clip_mode_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "剖切模式",
            TextStyle {
                font: asset_server.load("fonts/yysfont.ttf"),
                font_size: 25.0,
                color: Color::BLUE,
            },
        ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        ClipModeUi,
    ));
}
pub fn destroy_clip_mode_ui(mut commands: Commands, query: Query<Entity,With<ClipModeUi>>){
    query.for_each(|entity|{
        commands.entity(entity).despawn();
    })
}
pub struct ClipToolUiPlugin;
impl Plugin for ClipToolUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_clip_mode_ui.run_if(||{
            check_mode(Clip)
        })).add_system(destroy_clip_mode_ui.run_if(||{
            !check_mode(Clip)
        }))
        ;
    }
}