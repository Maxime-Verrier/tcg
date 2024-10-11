pub use bevy::prelude::*;
use bevy_replicon::core::Replicated;
use card_sim::{AgentOwned, Board, BoardSlot, Card, CardBundle, CardId, OnBoard, OnField, OnHand};
use epithet::{net::ClientInfo, units::UnitRegistry, utils::LevelEntity};

use crate::card::CardAssets;

pub fn create_dev_room_core_scene(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.3).looking_at(Vec3::ZERO, Vec3::Y),
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
    unit_registry: Res<UnitRegistry>,
    client_info: Res<ClientInfo>,
) {
    let agent = client_info.agent;
    let board = commands
        .spawn((
            Board::new(vec![agent]),
            Replicated,
            LevelEntity,
            Name::new("Board"),
        ))
        .id();
    commands.spawn((
        SpatialBundle::default(),
        BoardSlot(IVec3::new(0, 0, 0), None),
        LevelEntity,
        OnField,
        OnBoard(board),
        Replicated,
        AgentOwned(agent),
        Name::new("Slot"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: card_assets.deck_mesh.clone(),
            material: card_assets.deck_material.clone(),
            transform: Transform::from_xyz(0.5, 0.0, 0.0),
            ..default()
        },
        Name::new("Deck"),
        LevelEntity,
        OnBoard(board),
        AgentOwned(agent),
    ));

    for i in 0..3 {
        commands.spawn((
            CardBundle {
                card: Card(CardId(i % 2)),
                ..default()
            },
            OnBoard(board),
            OnHand,
            unit_registry.get_unit::<Card>(),
            AgentOwned(agent),
        ));
    }
}
