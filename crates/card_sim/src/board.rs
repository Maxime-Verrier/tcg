use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::{Component, DespawnRecursiveExt, Entity, OnRemove, Query, Trigger},
    utils::{HashMap, HashSet},
};
use epithet::agent::Agent;

use crate::{BoardSlot, FieldPosition, OnHand};

/// A component representing a board existing both as a marker and a lookup table to get entity on the board by common values
/// The reason for this lookup table exist is to reduce iteration when needing to get entities by x by value as we can't query entites just for x board/hand/player/etc..
/// ex: entities on this board, entities at x pos of the board, cards of x player etc
/// The lookup tables are automaticly synched internally when entites get their component removed/changed/added so no need to their values to the table
#[derive(Default, Debug)]
pub struct Board {
    // Lookup maps
    pos_lookup: HashMap<FieldPosition, Entity>,
    slots_lookup: HashMap<FieldPosition, Entity>,
    hand_lookup: HashMap<Entity, HashSet<Entity>>,

    // List of entity on the board and a player lookup at the same time
    // Not using a Vec with no associate value as it's will be mean reallocating everything and iteration are rarer than insert/remove, even if it's doesnt rly matter at the frequency of a card game
    board_lookup: HashSet<Entity>,
    player_lookup: HashMap<Entity, HashSet<Entity>>,
}

impl Board {
    fn insert_on_board(&mut self, entity: Entity) {
        self.board_lookup.insert(entity);
    }

    fn insert_by_agent(&mut self, agent: Entity, entity: Entity) {
        self.player_lookup.entry(agent).or_default().insert(entity);
    }

    pub(crate) fn insert_by_pos(&mut self, pos: FieldPosition, entity: Entity) {
        self.pos_lookup.insert(pos, entity);
    }

    pub(crate) fn insert_by_slot(&mut self, pos: FieldPosition, entity: Entity) {
        self.slots_lookup.insert(pos, entity);
    }

    pub(crate) fn insert_by_hand(&mut self, agent: Entity, entity: Entity) {
        self.hand_lookup.entry(agent).or_default().insert(entity);
    }

    fn remove_from_board(&mut self, entity: &Entity) -> bool {
        self.board_lookup.remove(entity)
    }

    pub(crate) fn remove_pos(&mut self, pos: &FieldPosition) -> Option<Entity> {
        self.pos_lookup.remove(pos)
    }

    pub(crate) fn remove_slot(&mut self, pos: &FieldPosition) -> Option<Entity> {
        self.slots_lookup.remove(pos)
    }

    fn remove_agent(&mut self, agent: Entity, entity: &Entity) -> bool {
        self.player_lookup
            .get_mut(&agent)
            .map_or(false, |entities| entities.remove(entity))
    }

    pub(crate) fn remove_hand(&mut self, agent: Entity, entity: &Entity) -> bool {
        self.hand_lookup
            .get_mut(&agent)
            .map_or(false, |entities| entities.remove(entity))
    }

    pub(crate) fn clean_agent_values(&mut self, agent: Entity) {
        self.player_lookup.remove(&agent);
        self.hand_lookup.remove(&agent);
    }

    pub fn get_entities(&self) -> &HashSet<Entity> {
        &self.board_lookup
    }

    pub fn get_by_pos(&self, pos: &FieldPosition) -> Option<&Entity> {
        self.pos_lookup.get(pos)
    }

    pub fn get_by_agent(&self, agent: Entity) -> Option<&HashSet<Entity>> {
        self.player_lookup.get(&agent)
    }

    pub fn get_by_hand(&self, agent: Entity) -> Option<&HashSet<Entity>> {
        self.hand_lookup.get(&agent)
    }

    pub fn get_by_slot(&self, pos: &FieldPosition) -> Option<&Entity> {
        self.slots_lookup.get(pos)
    }
}

impl Component for Board {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, board_entity, _component_id| {
            if let Some(field) = world.get::<Board>(board_entity) {
                let entities: Vec<Entity> = field.board_lookup.iter().cloned().collect();

                for entity in entities {
                    world.commands().entity(entity).remove::<(
                        OnBoard,
                        OnHand,
                        BoardSlot,
                        FieldPosition,
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
        hooks.on_add(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let agent = world.get::<AgentOwned>(entity).cloned();
            let pos = world.get::<FieldPosition>(entity).cloned();
            let slot = world.get::<BoardSlot>(entity).cloned();
            let on_hand = world.get::<OnHand>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.insert_on_board(entity);

                    if let Some(agent) = agent {
                        board.insert_by_agent(agent.0, entity);
                        if on_hand.is_some() {
                            board.insert_by_hand(agent.0, entity);
                        }
                    }
                    if let Some(pos) = pos {
                        if let Some(_) = slot {
                            board.insert_by_slot(pos, entity);
                        } else {
                            board.insert_by_pos(pos, entity);
                        }
                    }
                }
            }
        });
        hooks.on_remove(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let agent = world.get::<AgentOwned>(entity).cloned();
            let pos = world.get::<FieldPosition>(entity).cloned();
            let slot = world.get::<BoardSlot>(entity).cloned();
            let on_hand = world.get::<OnHand>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.remove_from_board(&entity);

                    if let Some(agent) = agent {
                        if on_hand.is_some() {
                            board.remove_hand(agent.0, &entity);
                        }
                        board.remove_agent(agent.0, &entity);
                    }
                    if let Some(pos) = pos {
                        if let Some(_) = slot {
                            board.remove_slot(&pos);
                        } else {
                            board.remove_pos(&pos);
                        }
                    }
                }
            }
        });
    }
}

#[derive(Clone, Copy)]
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
                        board.remove_agent(agent.0, &entity);
                    }
                }
            }
        });
    }
}

pub(crate) fn board_agent_removed_observer(
    trigger: Trigger<OnRemove, Agent>,
    query: Query<&OnBoard>,
    mut boards: Query<&mut Board>,
) {
    if let Ok(on_board) = query.get(trigger.entity()) {
        if let Ok(mut board) = boards.get_mut(on_board.0) {
            board.clean_agent_values(trigger.entity());
        }
    }
}
