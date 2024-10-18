use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use bevy_replicon::prelude::{FromClient, SendMode, ToClients};
use epithet::{
    agent::{AgentBundle, AgentManager},
    net::AuthManager,
    units::UnitRegistry,
};
use serde::{Deserialize, Serialize};

use crate::Board;

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct BoardAgentJoin {
    pub board: Entity,
    pub agent: Entity,
}

impl BoardAgentJoin {
    pub fn new(board: Entity, agent: Entity) -> Self {
        Self { board, agent }
    }
}

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct ClientJoinBoardRequestPacket {
    pub board: Entity,
}

impl ClientJoinBoardRequestPacket {
    pub fn new(board: Entity) -> Self {
        Self { board }
    }
}

impl MapEntities for ClientJoinBoardRequestPacket {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.board = entity_mapper.map_entity(self.board);
    }
}

// RepliconObserver
// Render is after and rely on ordering but observer odn't have that
pub fn player_join_packet_system(
    mut commands: Commands,
    mut packets: EventReader<FromClient<ClientJoinBoardRequestPacket>>,
    mut agent_manager: ResMut<AgentManager>,
    auth_manager: Res<AuthManager>,
    mut boards: Query<&mut Board>,
    mut writer: EventWriter<ToClients<ClientJoinedBoardPacket>>,
    unit_registry: Res<UnitRegistry>,
) {
    for FromClient { client_id, event } in packets.read() {
        if let Some(auth_id) = auth_manager.get_auth_id(client_id) {
            let agent = commands.spawn(AgentBundle::default()).id();

            if let Ok(mut board) = boards.get_mut(event.board) {
                //TODO check already have an agent/or is on board, decide if i want to keep generic agent

                agent_manager.insert(*auth_id, agent);
                board.add_agent(agent);
                board.create_agent_board(agent, event.board, &mut commands, &unit_registry);
                commands.trigger(BoardAgentJoin::new(event.board, agent));

                writer.send(ToClients {
                    mode: SendMode::Direct(*client_id),
                    event: ClientJoinedBoardPacket::new(event.board, agent),
                });
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

/// Packet sent by the server to the client that joined a board only
#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct ClientJoinedBoardPacket {
    pub board: Entity,
    pub agent: Entity,
}

impl ClientJoinedBoardPacket {
    pub fn new(board: Entity, agent: Entity) -> Self {
        Self { board, agent }
    }
}

impl MapEntities for ClientJoinedBoardPacket {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.board = entity_mapper.map_entity(self.board);
        self.agent = entity_mapper.map_entity(self.agent);
    }
}

// RepliconObserver
pub fn player_joined_packet_system(
    mut packets: EventReader<ClientJoinedBoardPacket>,
    mut boards: Query<&mut Board>,
) {
    for ClientJoinedBoardPacket { board, agent } in packets.read() {
        if let Ok(mut board) = boards.get_mut(*board) {
            board.client_is_on_board = Some(*agent);
        } else {
            warn!("Server tried to send a player joined packet to a board that does not exist, this should not be possible");
        }
    }
}
