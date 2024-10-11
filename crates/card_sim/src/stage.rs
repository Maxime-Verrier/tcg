use bevy::ecs::entity::MapEntities;
pub use bevy::prelude::*;
use bevy_replicon::prelude::FromClient;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
pub enum Stage {
    #[default]
    Start,
    Main,
    End,
}

#[derive(Event, Serialize, Deserialize, Clone)]
pub struct StageChangePacket {
    pub stage: Stage,
    pub board: Entity,
}

impl StageChangePacket {
    pub fn new(stage: Stage, board: Entity) -> Self {
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
) {
    for FromClient { client_id, event } in stage_client_stage_packet.read() {}
}
