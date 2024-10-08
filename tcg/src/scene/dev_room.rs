use action::{Action, ActionExecuteEvent, ActionState};
use bevy::log::Level;
pub use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Listener, On}, PickableBundle,
};
use card_sim::{
    AgentOwned, Board, BoardSlot, Card, CardAssets, CardBundle, CardId, FieldPosition, OnBoard,
    OnField, OnHand,
};
use epithet::{agent::Agent, utils::LevelEntity};

use crate::card::{
    self, summon_action_cancel, summon_action_execute, summon_action_finish, SummonActionResource,
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
    let board = commands.spawn((Board::default(), LevelEntity)).id();
    let agent = commands
        .spawn((Agent, LevelEntity, ActionState::default()))
        .id();

    commands.spawn((
        SpatialBundle::default(),
        BoardSlot(None),
        LevelEntity,
        FieldPosition::new(0, 0, 0),
        OnBoard(board),
    ));
    commands.spawn((
        PbrBundle {
            mesh: card_assets.deck_mesh.clone(),
            material: card_assets.deck_material.clone(),
            transform: Transform::from_xyz(0.5, 0.0, 0.0),
            ..default()
        },
        Name::new("Deck".to_string()),
        LevelEntity,
        OnBoard(board),
        AgentOwned(agent),
    ));

    for _ in 0..1 {
        card_assets.insert_card_render(
            &mut commands.spawn((
                CardBundle {
                    card: Card(CardId(0)),
                    ..default()
                },
                OnBoard(board),
                OnHand,
                AgentOwned(agent),
                On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>,
                     mut commands: Commands,
                     actionners: Query<Entity, With<ActionState>>,
                     cards: Query<Entity, With<Card>>,
                     summon_resources: Res<SummonActionResource>| {
                        commands.trigger(ActionExecuteEvent(Action::new(
                            actionners.get_single().unwrap(),
                            vec![event.listener()],
                            summon_resources.execute_id,
                            summon_resources.finish_id,
                            Some(summon_resources.cancel_id),
                        )));
                    },
                ),
            )),
            &CardId(0),
        );
    }
}
