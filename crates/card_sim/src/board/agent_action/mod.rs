mod summon;
mod target;

use bevy_inspector_egui::egui::util::id_type_map::TypeId;
use epithet::agent;
pub use summon::*;
pub use target::*;

use bevy::{
    ecs::{entity::MapEntities, system::SystemId},
    prelude::*,
    utils::HashMap,
};
use bevy_replicon::{
    bincode,
    prelude::{ChannelKind, ServerEventAppExt},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub(crate) fn agent_action_plugin(app: &mut App) {
    app.init_resource::<AgentActionRegistry>();

    app.add_systems(Update, client_agent_action_packet_handler);
    app.add_mapped_server_event::<AgentActionPacket>(ChannelKind::Ordered);

    let id = app.register_system(target_agent_action_callback);
    app.world_mut()
        .get_resource_mut::<AgentActionRegistry>()
        .unwrap()
        .register::<TargetAgentAction>(id);
}

fn client_agent_action_packet_handler(
    mut commands: Commands,
    mut packets: EventReader<AgentActionPacket>,
    registry: Res<AgentActionRegistry>,
) {
    for packet in packets.read() {
        if let Err(e) = registry.deserialize_and_run(
            packet.state.agent,
            packet.state.board,
            packet.state.agent_action_id,
            &packet.state.data,
            &mut commands,
        ) {
            eprintln!("Failed to handle agent action packet: {:?}", e);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentActionId(u32);

pub trait AgentAction {}

pub struct AgentActionInput<T> {
    pub agent: Entity,
    pub board: Entity,
    pub agent_action_id: AgentActionId,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct AgentActionState {
    agent: Entity,
    board: Entity,
    data: Vec<u8>,
    agent_action_id: AgentActionId,
}

#[derive(Event, Serialize, Deserialize)]
pub struct AgentActionPacket {
    state: AgentActionState,
}

impl AgentActionPacket {
    pub fn from<T: AgentAction + 'static + Send + Sync + Serialize + DeserializeOwned>(
        agent: Entity,
        board: Entity,
        agent_action: T,
        agent_action_id: AgentActionId,
    ) -> Self {
        let data = bincode::serialize(&agent_action).unwrap();

        Self {
            state: AgentActionState {
                agent,
                board,
                data,
                agent_action_id,
            },
        }
    }
}

impl MapEntities for AgentActionPacket {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.state.agent = entity_mapper.map_entity(self.state.agent);
        self.state.board = entity_mapper.map_entity(self.state.board);
    }
}

impl AgentActionPacket {
    pub fn new<T: AgentAction + 'static + Send + Sync + Serialize + DeserializeOwned>(
        agent: Entity,
        board: Entity,
        agent_action: T,
        agent_action_id: AgentActionId,
    ) -> Self {
        let data = bincode::serialize(&agent_action).unwrap();

        Self {
            state: AgentActionState {
                agent,
                board,
                data,
                agent_action_id,
            },
        }
    }
}

trait AgentActionFactory {
    fn create(
        &self,
        data: &[u8],
    ) -> Result<Box<dyn AgentAction + 'static + Send + Sync>, Box<dyn std::error::Error>>;

    fn run(
        &self,
        agent: Entity,
        board: Entity,
        agent_action_id: AgentActionId,
        data: &[u8],
        commands: &mut Commands,
        system_id: Entity,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

struct GenericAgentActionFactory<T>(std::marker::PhantomData<T>)
where
    T: AgentAction + 'static + Send + Sync + Serialize + DeserializeOwned;

impl<T> AgentActionFactory for GenericAgentActionFactory<T>
where
    T: AgentAction + 'static + Send + Sync + Serialize + DeserializeOwned,
{
    fn create(
        &self,
        data: &[u8],
    ) -> Result<Box<dyn AgentAction + 'static + Send + Sync>, Box<dyn std::error::Error>> {
        Ok(Box::new(bincode::deserialize::<T>(data)?))
    }

    fn run(
        &self,
        agent: Entity,
        board: Entity,
        agent_action_id: AgentActionId,
        data: &[u8],
        commands: &mut Commands,
        system_id: Entity,
    ) -> Result<(), Box<dyn std::error::Error>> {
        commands.run_system_with_input(
            SystemId::<AgentActionInput<T>>::from_entity(system_id),
            AgentActionInput {
                agent,
                board,
                agent_action_id,
                data: bincode::deserialize::<T>(data)?,
            },
        );

        Ok(())
    }
}

#[derive(Resource, Default)]
pub struct AgentActionRegistry {
    map: HashMap<AgentActionId, (Box<dyn AgentActionFactory + 'static + Send + Sync>, Entity)>,
    typeid_map: HashMap<TypeId, AgentActionId>,
}

impl AgentActionRegistry {
    pub fn register<T: AgentAction + 'static + Send + Sync + Serialize + DeserializeOwned>(
        &mut self,
        callback: SystemId<AgentActionInput<T>>,
    ) -> AgentActionId {
        let id = AgentActionId(self.map.len() as u32);

        self.map.insert(
            id,
            (
                Box::new(GenericAgentActionFactory::<T>(std::marker::PhantomData)),
                callback.entity(),
            ),
        );
        self.typeid_map.insert(TypeId::of::<T>(), id);

        id
    }

    pub fn get_action_id<T: AgentAction + 'static + Send + Sync + Serialize + DeserializeOwned>(
        &self,
    ) -> Option<&AgentActionId> {
        self.typeid_map.get(&TypeId::of::<T>())
    }

    pub fn deserialize(
        &self,
        id: AgentActionId,
        data: &[u8],
    ) -> Result<Box<dyn AgentAction + 'static + Send + Sync>, Box<dyn std::error::Error>> {
        let factory = &self
            .map
            .get(&id)
            .ok_or_else(|| "AgentActionId not found".to_string())?
            .0;

        factory.create(data)
    }

    pub fn deserialize_and_run(
        &self,
        agent: Entity,
        board: Entity,
        id: AgentActionId,
        data: &[u8],
        commands: &mut Commands,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (factory, system_id) = self
            .map
            .get(&id)
            .ok_or_else(|| -> Box<dyn std::error::Error> { "AgentAction not registered".into() })?;

        factory.run(agent, board, id, data, commands, *system_id)
    }
}
