use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{AgentOwned, Board, BoardSlot};

use super::{AgentAction, AgentActionInput};

#[derive(Serialize, Deserialize, Debug)]
pub struct TargetAgentAction {}

impl TargetAgentAction {}

impl AgentAction for TargetAgentAction {}

pub(crate) fn target_agent_action_callback(
    input: In<AgentActionInput<TargetAgentAction>>,
    mut commands: Commands,
    boards: Query<&Board>,
    slots_agents: Query<&AgentOwned, With<BoardSlot>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    println!("Target agent action callback");
    let board = match boards.get(input.board) {
        Ok(board) => board,
        Err(_) => {
            error!("No matching board entity found in the summon action input");
            return;
        }
    };
    for slot in board.cache.get_slots() {
        if true
        /*slot_agent.0 == client_agent*/ /* Modify when client know which agent he play */
        {
            commands
                .entity(*slot.1)
                .insert(On::<Pointer<Click>>::run(
                    move |event: Listener<Pointer<Click>>, mut commands: Commands| {
                        commands.entity(event.listener()).despawn_descendants();
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.1, 0.1, 0.1))),
                        ..default()
                    },));
                });
        }
    }
}
