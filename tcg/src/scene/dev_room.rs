pub use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_replicon::{client::ClientSet, core::Replicated};
use card_sim::{
    agent_action::{BoardAgentJoinTrigger, PlayerJoinPacket},
    Board, BoardStage, StageChangePacket,
};
use epithet::{net::AuthEvent, utils::LevelEntity};

use crate::card::CardAssets;

pub(crate) fn dev_room_plugin(app: &mut App) {
    app.add_systems(Update, on_client_devroom_scene.after(ClientSet::Receive));

    app.observe(on_board_agent_join);
}

pub fn on_board_agent_join(trigger: Trigger<BoardAgentJoinTrigger>, mut boards: Query<&mut Board>) {
    if let Ok(mut board) = boards.get_mut(trigger.event().board) {
        board.state.game_start();
    } else {
        error!(
            "Board {:?} on agent join, board not found, this should be a impossible state",
            trigger.entity()
        );
    }
}

pub fn on_client_devroom_scene(
    mut auth_packets: EventReader<AuthEvent>,
    mut writer: EventWriter<PlayerJoinPacket>,
    boards: Query<Entity, With<Board>>,
) {
    for _packet in auth_packets.read() {
        writer.send(PlayerJoinPacket::new(boards.single()));
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
