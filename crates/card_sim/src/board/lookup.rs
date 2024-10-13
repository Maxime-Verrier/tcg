use std::collections::BTreeSet;

use bevy::{
    prelude::*,
    utils::{hashbrown::HashSet, HashMap},
};
use serde::{Deserialize, Serialize};

#[derive(Reflect, Default, Debug)]
pub struct BoardLookup {
    // Lookup maps
    pub(crate) slots_lookup: HashMap<IVec3, Entity>,
    pub(crate) on_slot_lookup: HashMap<IVec3, Entity>,

    /// The key is the agent entity and the value is a set of entity on their hand
    /// BTreeSet is used as the order of the cards in the hand is important
    pub on_hand_lookup: HashMap<Entity, BTreeSet<Entity>>,
    pub(crate) on_board_lookup: HashSet<Entity>,

    /// Every entities that belong to a agent
    pub(crate) agent_lookup: HashMap<Entity, HashSet<Entity>>,

    pub(crate) on_field_lookup: HashSet<Entity>,
}

impl BoardLookup {
    // All the insert/remove functions that update the lookup table are private or pub(crate) cause the crate already automaticly call them when the component is added/removed
    pub(crate) fn insert_on_board(&mut self, entity: Entity) {
        self.on_board_lookup.insert(entity);
    }

    pub(crate) fn insert_by_agent(&mut self, agent: Entity, entity: Entity) {
        self.agent_lookup.entry(agent).or_default().insert(entity);
    }

    pub(crate) fn remove_from_board(&mut self, entity: &Entity) -> bool {
        self.on_board_lookup.remove(entity)
    }

    pub(crate) fn remove_from_agent(&mut self, agent: Entity, entity: &Entity) -> bool {
        self.agent_lookup
            .get_mut(&agent)
            .map_or(false, |entities| entities.remove(entity))
    }

    pub(crate) fn clean_agent_associate_values(&mut self, agent: Entity) {
        println!("cleaning agent associate values: {:?}", agent);
        self.agent_lookup.remove(&agent);
        self.on_hand_lookup.remove(&agent);
    }

    pub fn get_entities(&self) -> &HashSet<Entity> {
        &self.on_board_lookup
    }

    pub fn get_by_agent(&self, agent: Entity) -> Option<&HashSet<Entity>> {
        self.agent_lookup.get(&agent)
    }
}
