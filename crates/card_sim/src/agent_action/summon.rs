use bevy::ecs::entity::MapEntities;
pub use bevy::prelude::*;
use bevy_replicon::prelude::FromClient;
use epithet::{agent::AgentManager, net::AuthManager};
use serde::{Deserialize, Serialize};

use crate::{Board, BoardSlot, OnHand, OnSlot};

//TODO add controller interdediate as this is a trust the client event
#[derive(Event, Clone, Serialize, Deserialize, Debug)]
pub struct AgentSummonEvent {
    pub board_entity: Entity,
    pub card_entity: Entity,
    pub slot_entity: Entity,
}

impl AgentSummonEvent {
    pub fn new(board_entity: Entity, summoned_entity: Entity, slot_entity: Entity) -> Self {
        Self {
            board_entity,
            card_entity: summoned_entity,
            slot_entity,
        }
    }
}

impl MapEntities for AgentSummonEvent {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.board_entity = entity_mapper.map_entity(self.board_entity);
        self.card_entity = entity_mapper.map_entity(self.card_entity);
        self.slot_entity = entity_mapper.map_entity(self.slot_entity);
    }
}

//TODO make it an oberver and make a packet an interdiate to avoid trust the client and have ai being agent
pub(crate) fn summon_packet_system(
    mut commands: Commands,
    mut events: EventReader<FromClient<AgentSummonEvent>>,
    boards: Query<&mut Board>,
    slots: Query<&mut BoardSlot>,
    auth_manager: Res<AuthManager>,
    agent_manager: Res<AgentManager>,
) {
    for FromClient { client_id, event } in events.read() {
        if let Some(agent) = agent_manager.agent_from_client_id(client_id, &auth_manager) {
            if let Ok(board) = boards.get(event.board_entity) {
                if board.state.get_current_turn_agent() != agent {
                    warn!("Client {:?} tried to summon without being the current turn agent of this board", client_id);
                    continue;
                }
                if slots.contains(event.slot_entity) && board.is_on_field(event.slot_entity) {
                    if let Some(mut summoned_entity) = commands.get_entity(event.card_entity) {
                        summoned_entity.remove::<OnHand>();
                        summoned_entity.insert(OnSlot(event.slot_entity));
                    }
                }
            }
        } else {
            warn!(
                "Client {:?} tried to summon without having an agent",
                client_id
            );
        }
    }
}
