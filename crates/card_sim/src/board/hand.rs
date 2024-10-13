use std::collections::BTreeSet;

use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{AgentOwned, Board, OnBoard};

use super::BoardLookup;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct OnHand;

impl Component for OnHand {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let player = world.get::<AgentOwned>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(player) = player {
                        println!("inserting on hand: {:?}", entity);
                        board.lookup.insert_on_hand(player.0, entity);
                    }
                }
            }
        });
        hooks.on_remove(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let player = world.get::<AgentOwned>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(player) = player {
                        board.lookup.remove_from_hand(player.0, &entity);
                    }
                }
            }
        });
    }
}

impl BoardLookup {
    pub(crate) fn insert_on_hand(&mut self, agent: Entity, entity: Entity) {
        self.on_hand_lookup.entry(agent).or_default().insert(entity);
    }

    pub(crate) fn remove_from_hand(&mut self, agent: Entity, entity: &Entity) -> bool {
        println!("removing from hand: {:?}", entity);
        self.on_hand_lookup
            .get_mut(&agent)
            .map_or(false, |entities| entities.remove(entity))
    }

    pub fn get_by_hand(&self, agent: &Entity) -> Option<&BTreeSet<Entity>> {
        println!("{:?}{:?}", agent, self.on_hand_lookup);

        self.on_hand_lookup.get(agent)
    }
}
