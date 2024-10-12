use bevy::ecs::entity::MapEntities;
pub use bevy::prelude::*;
use bevy_replicon::prelude::FromClient;
use epithet::{agent::AgentManager, net::AuthManager};
use serde::{Deserialize, Serialize};

use super::{Board, BoardState};

#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
pub enum BoardStage {
    #[default]
    Start,
    Main,
    End,
}

/// Advances the board state to the specified stage.
///
/// # Arguments
///
/// * `stage` - The target stage to advance to.
///
/// # Returns
///
/// * `bool` - Always returns `true`.
impl BoardState {
    pub fn advance_stage(&mut self, stage: BoardStage) -> bool {
        if self.agents.is_empty() {
            error!("Cannot advance stage on a board with no agents, this function should never be called in this state");
            return false;
        }

        //TODO make chain stage when the target stage result in multiple stage change to trigger effects on each stage
        if stage == BoardStage::Start {
            self.current_turn_agent_index = if self.current_turn_agent_index < self.agents.len() - 1
            {
                self.current_turn_agent_index + 1
            } else {
                0
            };
            self.current_turn_agent = Some(self.agents[self.current_turn_agent_index]);
        }
        true
    }
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct StageChangePacket {
    pub stage: BoardStage,
    pub board: Entity,
}

impl StageChangePacket {
    pub fn new(stage: BoardStage, board: Entity) -> Self {
        Self { stage, board }
    }
}

impl MapEntities for StageChangePacket {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.board = entity_mapper.map_entity(self.board);
    }
}

pub(crate) fn stage_client_stage_packet_system(
    mut stage_client_stage_packet: EventReader<FromClient<StageChangePacket>>,
    auth_manager: Res<AuthManager>,
    agent_manager: Res<AgentManager>,
    mut boards: Query<&mut Board>,
) {
    let auth_manager = auth_manager.into_inner();

    for FromClient { client_id, event } in stage_client_stage_packet.read() {
        if let Some(agent) = agent_manager.agent_from_client_id(client_id, auth_manager) {
            if let Ok(mut board) = boards.get_mut(event.board) {
                if board
                    .state
                    .get_current_turn_agent()
                    .map_or(false, |current_agent| current_agent == *agent)
                {
                    if board.state.advance_stage(event.stage.clone()) {
                        println!("Client {:?} changed stage to {:?}", client_id, event.stage);
                        //TODO send stage change to all clients
                    } else {
                        warn!("Client {:?} tried to change stage to {:?} but it was a invalid stage target", client_id, event.stage);
                    }
                } else {
                    warn!("Client {:?} tried to change stage while not being the current turn agent of the board {:?}", client_id, event.board);
                }
            }
        } else {
            warn!(
                "Client {:?} is not authenticated or do not have an agent",
                client_id
            );
        }
    }
}
