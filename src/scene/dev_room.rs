pub use bevy::prelude::*;
use card::{Board, Card, CardAssets, CardBundle, CardId, OnBoard, OnField, OnHand, Player};
use epithet::utils::LevelEntity;

pub fn create_dev_room_core_scene(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 4.5, 3.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        LevelEntity,
    ));
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 80.0,
    });
}

pub fn create_dev_room_scene(
    mut commands: Commands,
    card_assets: Res<CardAssets>,
    camera: Query<Entity, With<Camera>>,
) {
    let board = commands.spawn((Board::default(), LevelEntity)).id();

    card_assets.insert_card_render(
        &mut commands.spawn((
            CardBundle {
                card: Card(CardId(0)),
                ..default()
            },
            OnBoard(board),
            OnField,
            TransformBundle::default(),
            Player(0)
        )),
        &CardId(0),
    );

    for i in 0..5 {
        card_assets.insert_card_render(
            &mut commands.spawn((
                CardBundle {
                    card: Card(CardId(0)),
                    ..default()
                },
                OnBoard(board),
                OnHand,
                TransformBundle::default(),
                Player(0)
            )),
            &CardId(0),
        );
    }
}
