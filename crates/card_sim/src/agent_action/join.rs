use bevy::ecs::entity::MapEntities;
pub use bevy::prelude::*;
use bevy_replicon::prelude::FromClient;
use epithet::{
    agent::{AgentBundle, AgentManager},
    net::AuthManager,
    units::UnitRegistry,
};
use serde::{Deserialize, Serialize};

use crate::Board;

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct BoardAgentJoinTrigger {
    pub board: Entity,
    pub agent: Entity,
}

impl BoardAgentJoinTrigger {
    pub fn new(board: Entity, agent: Entity) -> Self {
        Self { board, agent }
    }
}

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct PlayerJoinPacket {
    pub board: Entity,
}

impl PlayerJoinPacket {
    pub fn new(board: Entity) -> Self {
        Self { board }
    }
}

impl MapEntities for PlayerJoinPacket {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.board = entity_mapper.map_entity(self.board);
    }
}

// RepliconObserver
pub(crate) fn player_join_packet_system(
    mut commands: Commands,
    mut packets: EventReader<FromClient<PlayerJoinPacket>>,
    mut agent_manager: ResMut<AgentManager>,
    auth_manager: Res<AuthManager>,
    mut boards: Query<&mut Board>,
    unit_registry: Res<UnitRegistry>,
) {
    let unit_registry = unit_registry.into_inner();

    for FromClient { client_id, event } in packets.read() {
        if let Some(auth_id) = auth_manager.get_auth_id(client_id) {
            let agent = commands.spawn(AgentBundle::default()).id();

            if let Ok(mut board) = boards.get_mut(event.board) {
                //TODO check already have an agent/or is on board, decide if i want to keep generic agent
                agent_manager.insert(*auth_id, agent);
                board.add_agent(agent);
                board.create_agent_board(agent, event.board, &mut commands, unit_registry);
                commands.trigger(BoardAgentJoinTrigger::new(event.board, agent));
            } else {
                warn!(
                    "Client {:?} tried to join a board that does not exist",
                    event.board
                );
            }
        } else {
            warn!(
                "Client {:?} tried to join a board while not being auth",
                event.board
            );
        }
    }
}
