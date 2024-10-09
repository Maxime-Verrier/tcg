use bevy::ecs::component::{ComponentHooks, StorageType};
pub use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Board, OnBoard};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct OnField;

impl Component for OnField {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            if let Some(board_entity) = world.get::<OnBoard>(entity).cloned() {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.insert_on_field(entity);
                }
            }
        });
        hooks.on_remove(|mut world, entity, _component_id| {
            if let Some(board_entity) = world.get::<OnBoard>(entity).cloned() {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.remove_from_field(&entity);
                }
            }
        });
    }
}
