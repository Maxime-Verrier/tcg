mod action;
mod field;
mod hand;
mod lookup;
mod query;
mod slot;
mod stage;
mod state;
mod tree;

pub use action::*;
use bevy_replicon::bincode;
use bevy_replicon::core::ctx::WriteCtx;
use bevy_replicon::core::replication_registry::rule_fns::DeserializeFn;
use bevy_replicon::core::replication_registry::rule_fns::RuleFns;
use bevy_replicon::core::Replicated;
use epithet::units::UnitRegistry;
use epithet::utils::LevelEntity;
pub use field::*;
pub use hand::*;
pub use lookup::*;
pub use query::*;
pub use slot::*;
pub use stage::*;
pub use state::*;
pub use tree::*;

use std::collections::BTreeSet;
use std::io::Cursor;

use bevy::prelude::Commands;
use bevy::prelude::With;
use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        entity::MapEntities,
    },
    math::IVec3,
    prelude::{Component, Entity, EntityMapper, Event, Observer, OnRemove, Query, Trigger},
    reflect::Reflect,
    utils::{hashbrown::HashSet, HashMap},
};

use epithet::{agent::Agent, net::ClientReplicateWorld};
use serde::{Deserialize, Serialize};

use crate::Card;
use crate::CardBundle;
use crate::CardId;

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
    pub lookup: BoardLookup,
}

impl Board {
    pub fn new(agents: Vec<Entity>) -> Self {
        Self {
            client_is_on_board: None,
            state: BoardState::new(agents),
            lookup: BoardLookup::default(),
        }
    }

    pub fn trigger_effect(&mut self, card: Entity) {
        self.state.trigger_effect(card);
    }

    pub(crate) fn create_agent_board(
        &self,
        agent: Entity,
        board_entity: Entity,
        commands: &mut Commands,
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
                let entities: Vec<Entity> = field.lookup.on_board_lookup.iter().cloned().collect();

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
                    board.lookup.insert_on_board(entity);

                    if let Some(agent) = agent {
                        board.lookup.insert_by_agent(agent.0, entity);
                        if on_hand.is_some() {
                            board.lookup.insert_on_hand(agent.0, entity);
                        }
                    }
                    if on_field.is_some() {
                        board.lookup.insert_on_field(entity);
                    }
                    if let Some(slot) = slot {
                        board.lookup.insert_slot(slot.0, entity);
                    }
                    if let Some(slot) = temp_on_slot_slot {
                        board.lookup.insert_on_slot(slot.0, entity);
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
                    board.lookup.remove_from_board(&entity);

                    if let Some(agent) = agent {
                        if on_hand.is_some() {
                            board.lookup.remove_from_hand(agent.0, &entity);
                        }
                        board.lookup.remove_from_agent(agent.0, &entity);
                    }
                    if let Some(slot) = slot {
                        board.lookup.remove_slot(&slot.0);
                    }
                    if on_field.is_some() {
                        board.lookup.remove_from_field(&entity);
                    }
                    if let Some(slot) = temp_on_slot_slot {
                        board.lookup.remove_from_slot(&slot.0);
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
                        board.lookup.insert_by_agent(agent.0, entity);
                        if on_hand.is_some() {
                            board.lookup.insert_on_hand(agent.0, entity);
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
                        board.lookup.remove_from_agent(agent.0, &entity);
                        if on_hand.is_some() {
                            board.lookup.remove_from_hand(agent.0, &entity);
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
            board.lookup.clean_agent_associate_values(trigger.entity());
        }
    }
}
