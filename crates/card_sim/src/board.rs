use bevy::{
    ecs::{component::{ComponentHooks, StorageType}, entity::MapEntities}, math::IVec3, prelude::{Component, Entity, EntityMapper, OnRemove, Query, Trigger}, scene::ron::Map, utils::{hashbrown::HashSet, HashMap}
};
use epithet::agent::Agent;
use serde::{Deserialize, Serialize};

use crate::{BoardSlot, BoardState, OnField, OnHand, OnSlot};

/// A component representing a board existing both as a marker and a lookup table to get entity on the board by common values
/// The reason for this lookup table exist is to reduce iteration when needing to get entities by x by value as we can't query entites just for x board/hand/player/etc..
/// ex: entities on this board, entities at x pos of the board, cards of x player etc
/// The lookup tables are automaticly synched internally when entites get their component removed/changed/added so no need to their values to the table
#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    state: BoardState,

    // Lookup maps
    #[serde(skip)]
    slots_lookup: HashMap<IVec3, Entity>,
    #[serde(skip)]
    on_slot_lookup: HashMap<IVec3, Entity>,

    /// The key is the agent entity and the value is a set of entity on their hand
    #[serde(skip)]
    on_hand_lookup: HashMap<Entity, HashSet<Entity>>,

    /// List of entity on the board and a player lookup at the same time
    /// Not using a Vec with no associate value as it's will be mean reallocating everything and iteration are rarer than insert/remove, even if it's doesnt rly matter at the frequency of a card game
    #[serde(skip)]
    on_board_lookup: HashSet<Entity>,

    /// Every entities that belong to a agent
    #[serde(skip)]
    agent_lookup: HashMap<Entity, HashSet<Entity>>,
    #[serde(skip)]
    on_field_lookup: HashSet<Entity>,
}

impl Board {
    pub fn new(agents: Vec<Entity>) -> Self {
        Self {
            state: BoardState::new(agents[0]),
            on_slot_lookup: HashMap::default(),
            on_field_lookup: HashSet::default(),
            slots_lookup: HashMap::default(),
            on_hand_lookup: HashMap::default(),
            on_board_lookup: HashSet::default(),
            agent_lookup: HashMap::default(),
        }
    }

    pub fn trigger_effect(&mut self, card: Entity) {
        self.state.trigger_effect(card);
    }

    // All the insert/remove functions that update the lookup table are private or pub(crate) cause the crate already automaticly call them when the component is added/removed
    fn insert_on_board(&mut self, entity: Entity) {
        self.on_board_lookup.insert(entity);
    }

    fn insert_by_agent(&mut self, agent: Entity, entity: Entity) {
        self.agent_lookup.entry(agent).or_default().insert(entity);
    }

    pub(crate) fn insert_on_field(&mut self, entity: Entity) {
        self.on_field_lookup.insert(entity);
    }

    pub(crate) fn insert_slot(&mut self, pos: IVec3, entity: Entity) {
        self.slots_lookup.insert(pos, entity);
    }

    pub fn insert_on_slot(&mut self, pos: IVec3, entity: Entity) {
        self.on_slot_lookup.insert(pos, entity);
    }

    pub(crate) fn insert_on_hand(&mut self, agent: Entity, entity: Entity) {
        self.on_hand_lookup.entry(agent).or_default().insert(entity);
    }

    fn remove_from_board(&mut self, entity: &Entity) -> bool {
        self.on_board_lookup.remove(entity)
    }

    pub(crate) fn remove_from_field(&mut self, entity: &Entity) -> bool {
        self.on_field_lookup.remove(entity)
    }

    pub(crate) fn remove_from_slot(&mut self, pos: &IVec3) -> Option<Entity> {
        self.on_slot_lookup.remove(pos)
    }

    pub(crate) fn remove_slot(&mut self, pos: &IVec3) -> Option<Entity> {
        self.slots_lookup.remove(pos)
    }

    fn remove_from_agent(&mut self, agent: Entity, entity: &Entity) -> bool {
        self.agent_lookup
            .get_mut(&agent)
            .map_or(false, |entities| entities.remove(entity))
    }

    pub(crate) fn remove_from_hand(&mut self, agent: Entity, entity: &Entity) -> bool {
        self.on_hand_lookup
            .get_mut(&agent)
            .map_or(false, |entities| entities.remove(entity))
    }

    pub(crate) fn clean_agent_associate_values(&mut self, agent: Entity) {
        self.agent_lookup.remove(&agent);
        self.on_hand_lookup.remove(&agent);
    }

    pub fn get_entities(&self) -> &HashSet<Entity> {
        &self.on_board_lookup
    }

    pub fn is_on_field(&self, entity: Entity) -> bool {
        self.on_field_lookup.contains(&entity)
    }

    pub fn get_entities_on_field(&self) -> &HashSet<Entity> {
        &self.on_field_lookup
    }

    pub fn get_slots(&self) -> &HashMap<IVec3, Entity> {
        &self.slots_lookup
    }

    pub fn get_by_agent(&self, agent: Entity) -> Option<&HashSet<Entity>> {
        self.agent_lookup.get(&agent)
    }

    pub fn get_by_hand(&self, agent: Entity) -> Option<&HashSet<Entity>> {
        self.on_hand_lookup.get(&agent)
    }

    pub fn get_on_slot(&self, pos: &IVec3) -> Option<&Entity> {
        self.slots_lookup.get(pos)
    }
}

impl Component for Board {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, board_entity, _component_id| {
            if let Some(field) = world.get::<Board>(board_entity) {
                let entities: Vec<Entity> = field.on_board_lookup.iter().cloned().collect();

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

/// Mark the entity as part of x board and add the relevent entity's values to the lookup table
#[derive(Clone, Copy)]
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
                    board.insert_on_board(entity);

                    if let Some(agent) = agent {
                        board.insert_by_agent(agent.0, entity);
                        if on_hand.is_some() {
                            board.insert_on_hand(agent.0, entity);
                        }
                    }
                    if on_field.is_some() {
                        board.insert_on_field(entity);
                    }
                    if let Some(slot) = slot {
                        board.insert_slot(slot.0, entity);
                    }
                    if let Some(slot) = temp_on_slot_slot {
                        board.insert_on_slot(slot.0, entity);
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
                    board.remove_from_board(&entity);

                    if let Some(agent) = agent {
                        if on_hand.is_some() {
                            board.remove_from_hand(agent.0, &entity);
                        }
                        board.remove_from_agent(agent.0, &entity);
                    }
                    if let Some(slot) = slot {
                        board.remove_slot(&slot.0);
                    }
                    if on_field.is_some() {
                        board.remove_from_field(&entity);
                    }
                    if let Some(slot) = temp_on_slot_slot {
                        board.remove_from_slot(&slot.0);
                    }
                }
            }
        });
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct AgentOwned(pub Entity);

impl Component for AgentOwned {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let agent = world.get::<AgentOwned>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(agent) = agent {
                        board.insert_by_agent(agent.0, entity);
                    }
                }
            }
        });

        hooks.on_remove(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let agent = world.get::<AgentOwned>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(agent) = agent {
                        board.remove_from_agent(agent.0, &entity);
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
    trigger: Trigger<OnRemove, Agent>,
    query: Query<&OnBoard>,
    mut boards: Query<&mut Board>,
) {
    //todo err println
    if let Ok(on_board) = query.get(trigger.entity()) {
        if let Ok(mut board) = boards.get_mut(on_board.0) {
            board.clean_agent_associate_values(trigger.entity());
        }
    }
}
