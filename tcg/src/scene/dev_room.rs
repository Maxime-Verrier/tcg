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

use crate::{
    action::{Action, ActionState},
    card::{SummonActionFinishEvent, SummonActionEvent},
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
                     mut action_states: Query<(&mut ActionState, Entity), With<Agent>>,
                     on_boards: Query<&OnBoard>| {
                        if let Ok(on_board) = on_boards.get(event.listener()) {
                            let (mut action_state, self_agent) = action_states.single_mut(); //TODO replace with get self agent ??? there should be only one self action_state anyway

                            action_state.execute_action(
                                &mut commands,
                                Action::new(
                                    self_agent,
                                    Box::new(SummonActionEvent::new(on_board.0, event.listener())),
                                    Box::new(SummonActionFinishEvent),
                                ),
                            );
                        }
                    },
                ),
            )),
            &CardId(i % 2),
        );
    }
}
