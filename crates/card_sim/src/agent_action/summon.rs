use bevy::ecs::entity::MapEntities;
pub use bevy::prelude::*;
use bevy_replicon::prelude::FromClient;
use epithet::{agent::AgentManager, net::AuthManager};
use serde::{Deserialize, Serialize};

use crate::{AgentOwned, Board, BoardSlot, OnHand, OnSlot};

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
    on_hands : Query<&AgentOwned, With<OnHand>>,
    auth_manager: Res<AuthManager>,
    agent_manager: Res<AgentManager>,
) {
    for FromClient { client_id, event } in events.read() {
        let agent = match agent_manager.agent_from_client_id(client_id, &auth_manager) {
            Some(agent) => agent,
            None => {
                warn!(
                    "Client {:?} tried to summon without having an agent",
                    client_id
                );
                continue;
            }
        };

        if let Ok(agent_owned) = on_hands.get(event.card_entity) {
            if agent_owned.0 != *agent {
                warn!("Client {:?} tried to summon a card that was on another agent hand", client_id);
                continue;
            }
        }
        else {
            warn!("Client {:?} tried to summon a card that was not on any hand", client_id);
            continue;
        }

        let board = match boards.get(event.board_entity) {
            Ok(board) => board,
            Err(_) => continue,
        };

        //TODO change later as you can summon without being the turn agent in the future, prio or smth like that ?
        if !board
            .state
            .get_current_turn_agent()
            .map_or(false, |current_agent| current_agent == *agent)
        {
            warn!("Client {:?} tried to summon without being the current turn agent on the board {:?}", client_id, event.board_entity);
            continue;
        }

        if !slots.contains(event.slot_entity) || !board.lookup.is_on_field(event.slot_entity) {
            warn!("Client {:?} tried to summon to a slot {:?} that does not exist or is not on the field on the board {:?}", client_id, event.slot_entity,  event.board_entity);
            continue;
        }

        if let Some(mut summoned_entity) = commands.get_entity(event.card_entity) {
            summoned_entity.remove::<OnHand>();
            summoned_entity.insert(OnSlot(event.slot_entity));
        } else {
            warn!("Client {:?} tried to summon a card that does not exist, on slot {:?}, on the board {:?}", client_id, event.slot_entity, event.board_entity);
        }
    }
}
