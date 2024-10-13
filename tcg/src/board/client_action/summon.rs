pub use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Listener, On},
};
use card_sim::BoardSlot;
use card_sim::{agent_action::AgentSummonEvent, AgentOwned, Board};

use super::ClientActionState;

#[derive(Event, Clone)]
pub struct ClientSummonAction {
    pub board_entity: Entity,
    pub summon_entity: Entity,
}

impl ClientSummonAction {
    pub fn new(board_entity: Entity, summon_entity: Entity) -> Self {
        Self {
            board_entity,
            summon_entity,
        }
    }
}

#[cfg(feature = "render")]
#[derive(Component)]
pub(crate) struct SummonActionFXMarker;

pub(crate) fn summon_action_execute(
    trigger: Trigger<ClientSummonAction>,
    mut commands: Commands,
    boards: Query<&Board>,
    slots_agents: Query<&AgentOwned, With<BoardSlot>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let summon_event = trigger.event().clone();

    let board = match boards.get(summon_event.board_entity) {
        Ok(board) => board,
        Err(_) => {
            error!("No matching board entity found in the summon action input");
            return;
        }
    };
    for slot in board.lookup.get_slots() {
        if true
        /*slot_agent.0 == client_agent*/ /* Modify when client know which agent he play */
        {
            commands
                .entity(*slot.1)
                .insert(On::<Pointer<Click>>::run(
                    move |event: Listener<Pointer<Click>>,
                          mut summon_packet_writer: EventWriter<AgentSummonEvent>,
                          mut action_state: ResMut<ClientActionState>,
                          mut commands: Commands| {
                        summon_packet_writer.send(AgentSummonEvent::new(
                            summon_event.board_entity,
                            summon_event.summon_entity,
                            event.listener(),
                        ));
                        commands.trigger(SummonActionFinishEvent);
                        action_state.current = None;
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(Cuboid::new(0.1, 0.1, 0.1))),
                            ..default()
                        },
                        SummonActionFXMarker,
                    ));
                });
        }
    }
    //TODO slot check to cancel the action ?
}

#[derive(Event, Clone)]
pub struct SummonActionFinishEvent;

pub(crate) fn summon_action_finish(
    _trigger: Trigger<SummonActionFinishEvent>,
    query: Query<Entity, With<SummonActionFXMarker>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
