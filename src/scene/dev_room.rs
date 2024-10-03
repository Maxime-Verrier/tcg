pub use bevy::prelude::*;
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
pub fn create_dev_room_scene() {

}