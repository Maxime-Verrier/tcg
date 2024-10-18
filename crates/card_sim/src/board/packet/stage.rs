use bevy::{ecs::entity::MapEntities, prelude::*};
use bevy_replicon::prelude::FromClient;
use epithet::{agent::AgentManager, net::AuthManager};
use serde::{Deserialize, Serialize};

use crate::{Board, BoardStage};

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
