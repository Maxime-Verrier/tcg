pub use bevy::prelude::*;
use card::{Board, OnBoard, OnField};
use epithet::utils::LevelEntity;

pub fn create_dev_room_core_scene(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        LevelEntity,
    ));
}
pub fn create_dev_room_scene(mut commands: Commands) {
    let board = commands.spawn((Board::default(), LevelEntity)).id();

    commands.spawn((OnBoard(board), OnField, LevelEntity));
}
