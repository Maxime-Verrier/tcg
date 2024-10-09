use action::{Action, ActionExecuteEvent, ActionState};
pub use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Listener, On},
};
use bevy_replicon::core::Replicated;
use card_sim::{
    AgentOwned, Board, BoardSlot, Card, CardAssets, CardBundle, CardId, OnBoard, OnField, OnHand,
};
use epithet::{agent::Agent, utils::LevelEntity};

use crate::card::{
    SummonActionResource,
};

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

pub fn create_dev_room_scene(mut commands: Commands, card_assets: Res<CardAssets>) {
    let agent = commands
        .spawn((Agent, LevelEntity, ActionState::default(), Replicated))
        .id();
    let board = commands
        .spawn((
            Board::new(vec![agent]),
            Replicated,
            LevelEntity,
            Name::new("Board"),
        ))
        .id();
    let slot = commands
        .spawn((
            SpatialBundle::default(),
            BoardSlot(IVec3::new(0, 0, 0), None),
            LevelEntity,
            OnField,
            OnBoard(board),
            Replicated,
            AgentOwned(agent),
            Name::new("Slot"),
        ))
        .id();

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

    for i in 0..4 {
        card_assets.insert_card_render(
            &mut commands.spawn((
                CardBundle {
                    card: Card(CardId(i)),
                    ..default()
                },
                OnBoard(board),
                OnHand,
                AgentOwned(agent),
                On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>,
                     mut commands: Commands,
                     actionners: Query<Entity, With<ActionState>>,
                     on_boards: Query<&OnBoard>,
                     summon_resources: Res<SummonActionResource>| {
                        if let Ok(on_board) = on_boards.get(event.listener()) {
                            commands.trigger(ActionExecuteEvent(Action::new(
                                actionners.get_single().unwrap(),
                                vec![event.listener(), on_board.0],
                                summon_resources.execute_id,
                                summon_resources.finish_id,
                                Some(summon_resources.cancel_id),
                            )));
                        }
                    },
                ),
            )),
            &CardId(i % 2),
        );
    }
}
