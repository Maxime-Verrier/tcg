pub use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_replicon::{client::ClientSet, core::Replicated};
use card_sim::{
    agent_action::{BoardAgentJoinTrigger, ClientJoinBoardRequestPacket},
    AgentOwned, Board, BoardStage, Card, CardBundle, CardId, OnBoard, OnHand, StageChangePacket,
};
use epithet::{net::AuthEvent, units::UnitRegistry, utils::LevelEntity};

use crate::card::CardAssets;

pub(crate) fn dev_room_plugin(app: &mut App) {
    app.add_systems(Update, on_client_devroom_scene);

    app.observe(on_board_agent_join);
}

pub fn on_board_agent_join(
    trigger: Trigger<BoardAgentJoinTrigger>,
    mut boards: Query<&mut Board>,
    mut commands: Commands,
    unit_registry: Res<UnitRegistry>,
) {
    if let Ok(mut board) = boards.get_mut(trigger.event().board) {
        board.state.game_start();

        for i in 0..5 {
            commands.spawn((
                CardBundle {
                    card: Card(CardId(i % 2)),
                    ..default()
                },
                OnBoard(trigger.event().board),
                OnHand,
                unit_registry.get_unit::<Card>(),
                AgentOwned(trigger.event().agent),
            ));
        }
    } else {
        error!(
            "Board {:?} on agent join, board not found, this should be a impossible state",
            trigger.entity()
        );
    }
}

pub fn on_client_devroom_scene(
    mut auth_packets: EventReader<AuthEvent>,
    mut writer: EventWriter<ClientJoinBoardRequestPacket>,
    boards: Query<Entity, With<Board>>,
) {
    for _packet in auth_packets.read() {
        writer.send(ClientJoinBoardRequestPacket::new(boards.single()));
    }
}

pub fn create_dev_room_core_scene(mut commands: Commands, card_assets: Res<CardAssets>) {
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

pub fn create_dev_room_scene(mut commands: Commands, card_assets: Res<CardAssets>) {
    let board = commands
        .spawn((
            Board::new(vec![]),
            Replicated,
            LevelEntity,
            Name::new("Board"),
        ))
        .id();

    //TODO should be on the clients only
    commands.spawn((
        PbrBundle {
            mesh: card_assets.deck_mesh.clone(),
            material: card_assets.deck_material.clone(),
            transform: Transform::from_xyz(0.5, 0.0, 0.0),
            ..default()
        },
        Name::new("Turn Button"),
        LevelEntity,
        On::<Pointer<Click>>::run(
            move |event: Listener<Pointer<Click>>, mut writer: EventWriter<StageChangePacket>| {
                writer.send(StageChangePacket::new(BoardStage::Start, board));
            },
        ),
    ));
}
