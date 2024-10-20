mod agent_action;
mod cache;
mod field;
mod hand;
mod packet;
mod query;
mod sequence;
mod slot;
mod stage;
mod state;
mod tree;

pub use agent_action::*;
pub use cache::*;
pub use field::*;
pub use hand::*;
pub use packet::*;
pub use query::*;
pub use sequence::*;
pub use slot::*;
pub use stage::*;
pub use state::*;
pub use tree::*;

use std::io::Cursor;

use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        entity::MapEntities,
    },
    math::IVec3,
    prelude::*,
    prelude::{Component, Entity, EntityMapper, OnRemove, Query, Trigger},
    reflect::Reflect,
};
use bevy_replicon::{
    bincode,
    core::{
        ctx::WriteCtx,
        replication_registry::rule_fns::{DeserializeFn, RuleFns},
    },
    prelude::*,
};
use epithet::agent::Agent;
use epithet::units::UnitRegistry;
use epithet::utils::LevelEntity;
use serde::{Deserialize, Serialize};

pub(crate) fn board_plugin(app: &mut App) {
    app.add_plugins((board_packet_plugin, agent_action_plugin));

    app.register_type::<Board>();
    app.register_type::<OnSlot>();

    app.replicate_with::<Board>(
        RuleFns::default_mapped().with_in_place(Board::board_in_place_as_deserialize),
    );
    app.replicate_mapped::<OnBoard>();
    app.replicate::<OnHand>();
    app.replicate::<OnField>();
    app.replicate_mapped::<AgentOwned>();
    app.replicate_mapped::<OnSlot>();
    app.replicate_mapped::<BoardSlot>();

    app.add_systems(Update, board_state_update);

    app.observe(board_agent_removed_observer);
}

/// A component representing a board existing both as a marker and a lookup table to get entity on the board by common values
/// The reason for this lookup table exist is to reduce iteration when needing to get entities by x by value as we can't query entites just for x board/hand/player/etc..
/// ex: entities on this board, entities at x pos of the board, cards of x player etc
/// The lookup tables are automaticly synched internally when entites get their component removed/changed/added so no need to their values to the table
#[derive(Reflect, Serialize, Deserialize, Debug)]
pub struct Board {
    /// Tell toward the app running the simulation if the app is a client and is playing on this board
    /// None if he is not playing on this board, the entity is the agent entity
    #[serde(skip)]
    pub client_is_on_board: Option<Entity>,

    pub state: BoardState,

    #[serde(skip)]
    pub cache: BoardCache,
}

impl Board {
    pub fn new(agents: Vec<Entity>) -> Self {
        Self {
            client_is_on_board: None,
            state: BoardState::new(agents),
            cache: BoardCache::default(),
        }
    }

    pub fn trigger_effect(&mut self, card: Entity, effect_index: usize) {
        self.state.trigger_effect(card, effect_index);
    }

    pub(crate) fn create_agent_board(
        &self,
        agent: Entity,
        board_entity: Entity,
        commands: &mut Commands,
        unit_registry: &UnitRegistry,
    ) {
        //TODO put it in the sim
        commands.spawn((
            SpatialBundle::default(),
            BoardSlot(IVec3::new(0, 0, 0), None),
            LevelEntity,
            OnField,
            OnBoard(board_entity),
            Replicated,
            AgentOwned(agent),
            unit_registry.get_unit::<BoardSlot>(),
            Name::new("Slot"),
        ));
    }

    pub fn board_in_place_as_deserialize(
        deserialize: DeserializeFn<Board>,
        ctx: &mut WriteCtx,
        component: &mut Board,
        cursor: &mut Cursor<&[u8]>,
    ) -> bincode::Result<()> {
        let deserialized_board = (deserialize)(ctx, cursor)?;

        component.state.agents = deserialized_board.state.agents;
        component.state.current_turn_agent = deserialized_board.state.current_turn_agent;
        component.state.current_turn_agent_index =
            deserialized_board.state.current_turn_agent_index;
        component.state.stage = deserialized_board.state.stage;

        Ok(())
    }
}

impl Component for Board {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, board_entity, _component_id| {
            if let Some(field) = world.get::<Board>(board_entity) {
                let entities: Vec<Entity> = field.cache.on_board_lookup.iter().cloned().collect();

                for entity in entities {
                    world.commands().entity(entity).remove::<(
                        //TODO add as children all entities instead ?
                        OnBoard,
                        OnHand,
                        BoardSlot,
                        OnField,
                        OnSlot,
                        AgentOwned,
                    )>();
                }
            }
        });
    }
}

impl MapEntities for Board {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.state.map_entities(entity_mapper);
    }
}

/// Mark the entity as part of x board and add the relevent entity's values to the lookup table
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct OnBoard(pub Entity);

impl Component for OnBoard {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let agent = world.get::<AgentOwned>(entity).cloned();
            let on_field = world.get::<OnField>(entity).cloned();
            let slot = world.get::<BoardSlot>(entity).cloned();
            let on_slot = world.get::<OnSlot>(entity).cloned();
            let on_hand = world.get::<OnHand>(entity).cloned();

            let temp_on_slot_slot = if let Some(on_slot) = &on_slot {
                //TODO modify when we will be able to avoid clone with multiple get with system_state in deffered world
                world.get::<BoardSlot>(on_slot.0).cloned()
            } else {
                None
            };

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.cache.insert_on_board(entity);

                    if let Some(agent) = agent {
                        board.cache.insert_by_agent(agent.0, entity);
                        if on_hand.is_some() {
                            board.cache.insert_on_hand(agent.0, entity);
                        }
                    }
                    if on_field.is_some() {
                        board.cache.insert_on_field(entity);
                    }
                    if let Some(slot) = slot {
                        board.cache.insert_slot(slot.0, entity);
                    }
                    if let Some(slot) = temp_on_slot_slot {
                        board.cache.insert_on_slot(slot.0, entity);
                    }
                }
            }
        });
        hooks.on_remove(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let agent = world.get::<AgentOwned>(entity).cloned();
            let on_field = world.get::<OnField>(entity).cloned();
            let slot = world.get::<BoardSlot>(entity).cloned();
            let on_hand = world.get::<OnHand>(entity).cloned();
            let on_slot = world.get::<OnSlot>(entity).cloned();

            let temp_on_slot_slot = if let Some(on_slot) = &on_slot {
                //TODO modify when we will be able to avoid clone with multiple get with system_state in deffered world
                world.get::<BoardSlot>(on_slot.0).cloned()
            } else {
                None
            };

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.cache.remove_from_board(&entity);

                    if let Some(agent) = agent {
                        if on_hand.is_some() {
                            board.cache.remove_from_hand(agent.0, &entity);
                        }
                        board.cache.remove_from_agent(agent.0, &entity);
                    }
                    if let Some(slot) = slot {
                        board.cache.remove_slot(&slot.0);
                    }
                    if on_field.is_some() {
                        board.cache.remove_from_field(&entity);
                    }
                    if let Some(slot) = temp_on_slot_slot {
                        board.cache.remove_from_slot(&slot.0);
                    }
                }
            }
        });
    }
}

impl MapEntities for OnBoard {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.0 = entity_mapper.map_entity(self.0);
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct AgentOwned(pub Entity);

impl Component for AgentOwned {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let agent = world.get::<AgentOwned>(entity).cloned();
            let on_hand = world.get::<OnHand>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(agent) = agent {
                        board.cache.insert_by_agent(agent.0, entity);
                        if on_hand.is_some() {
                            board.cache.insert_on_hand(agent.0, entity);
                        }
                    }
                }
            }
        });

        hooks.on_remove(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let agent = world.get::<AgentOwned>(entity).cloned();
            let on_hand = world.get::<OnHand>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(agent) = agent {
                        board.cache.remove_from_agent(agent.0, &entity);
                        if on_hand.is_some() {
                            board.cache.remove_from_hand(agent.0, &entity);
                        }
                    }
                }
            }
        });
    }
}

impl MapEntities for AgentOwned {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.0 = entity_mapper.map_entity(self.0);
    }
}

pub(crate) fn board_agent_removed_observer(
    //TODO do the same for slot ?
    //TODO check if only one player and win/draw by default
    //TODO only on agent on board ? idk
    trigger: Trigger<OnRemove, Agent>,
    query: Query<&OnBoard>,
    mut boards: Query<&mut Board>,
) {
    //TODO err print
    if let Ok(on_board) = query.get(trigger.entity()) {
        if let Ok(mut board) = boards.get_mut(on_board.0) {
            board.cache.clean_agent_associate_values(trigger.entity());
        }
    }
}
