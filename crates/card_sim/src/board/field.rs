use bevy::prelude::*;
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    utils::HashSet,
};
use serde::{Deserialize, Serialize};

use crate::{Board, OnBoard};

use super::BoardLookup;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct OnField;

impl Component for OnField {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            if let Some(board_entity) = world.get::<OnBoard>(entity).cloned() {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.lookup.insert_on_field(entity);
                }
            }
        });
        hooks.on_remove(|mut world, entity, _component_id| {
            if let Some(board_entity) = world.get::<OnBoard>(entity).cloned() {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.lookup.remove_from_field(&entity);
                }
            }
        });
    }
}

impl BoardLookup {
    pub(crate) fn insert_on_field(&mut self, entity: Entity) {
        self.on_field_lookup.insert(entity);
    }

    pub(crate) fn remove_from_field(&mut self, entity: &Entity) -> bool {
        self.on_field_lookup.remove(entity)
    }

    pub fn is_on_field(&self, entity: Entity) -> bool {
        self.on_field_lookup.contains(&entity)
    }

    pub fn get_entities_on_field(&self) -> &HashSet<Entity> {
        &self.on_field_lookup
    }
}
