use action::{ActionFinishEvent, ActionInput, ActionState};
use bevy::ecs::system::SystemId;
pub use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Listener, On},
};
use card_sim::{agent_action::AgentSummonEvent, AgentOwned, Board};
use card_sim::{BoardSlot, OnHand};

#[derive(Resource)]
pub struct SummonActionResource {
    pub execute_id: SystemId<ActionInput>,
    pub cancel_id: SystemId<ActionInput>,
    pub finish_id: SystemId<ActionInput>,
}

pub fn summon_action_execute(
    input: In<ActionInput>,
    mut commands: Commands,
    boards: Query<&Board>,
    slots_agents: Query<&AgentOwned, With<BoardSlot>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut summon_packet_writer: EventWriter<AgentSummonEvent>,
) {
    let action_input = input.0;

    if action_input.entities.len() < 2 {
        error!("Invalid number of arguments in action summon input");
        return;
    }

    let summon_entity = action_input.entities[0];
    let board_entity = action_input.entities[1];

    let board = match boards.get(board_entity) {
        Ok(board) => board,
        Err(_) => {
            error!("No matching board entity found in the summon action input");
            return;
        }
    };

    for slot in board.get_slots() {
        let slot_agent = match slots_agents.get(*slot.1) {
            Ok(slot_agent) => slot_agent,
            Err(_) => {
                error!("No agent found in slot from board, this is a board state error, this should never happen, check the board invariants/observers/hooks");
                continue;
            }
        };

        if true
        /*slot_agent.0 == client_agent*/ /* Modify when client know which agent he play */
        {
            commands
                .entity(*slot.1)
                .insert((On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>,
                    mut commands: Commands,
                    actionners: Query<Entity, With<ActionState>>| {
                        commands.trigger_targets(ActionFinishEvent, actionners.get_single().unwrap());
                    },
                ),))
                .with_children(|parent| {
                    parent.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.1, 0.1, 0.1))),
                        ..default()
                    });
                });
            summon_packet_writer.send(AgentSummonEvent::new(
                board_entity,
                summon_entity,
                *slot.1,
            ));
        }
    }
    //TODO slot check to avoid soft lock
}

pub fn summon_action_cancel(input: In<ActionInput>) {}

pub fn summon_action_finish(
    input: In<ActionInput>,
    mut commands: Commands,
    slots: Query<(Entity, &Transform), With<BoardSlot>>,
    mut transforms: Query<&mut Transform, Without<BoardSlot>>,
) {
    let action_input = input.0;

    if action_input.entities.len() < 2 {
        error!("Invalid number of arguments in action finish input");
        return;
    }

    let summon_entity = action_input.entities[0];

    if let Ok((slot_entity, slot_transform)) = slots.get_single() /*TODO OMG just fix this already */ {
        commands.entity(slot_entity).despawn_descendants(); //TODO remove when better fx cause just removing childrens is dumb and can have sideffect
        if let Ok(mut summon_transform) = transforms.get_mut(summon_entity) {
            if let Some(mut entity) = commands.get_entity(summon_entity) {
                entity.remove::<(On<Pointer<Click>>, OnHand)>();
                summon_transform.translation = slot_transform.translation;
                summon_transform.rotation = Quat::from_rotation_x(90.0_f32.to_radians())
                    * Quat::from_rotation_z(180.0_f32.to_radians());
            }
        }
    } else {
        error!("No single slot transform found");
    }
}
