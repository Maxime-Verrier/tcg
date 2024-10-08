use bevy::ecs::component::{ComponentHooks, StorageType};
pub use bevy::prelude::*;

use crate::{Board, BoardSlot, OnBoard};

#[derive(Component)]
pub struct OnField;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldPosition(pub IVec3);

impl FieldPosition {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }
}

impl Component for FieldPosition {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let pos = world.get::<FieldPosition>(entity).cloned();
            let slot = world.get::<BoardSlot>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(pos) = pos {
                        if let Some(slot) = slot {
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
            let pos: Option<FieldPosition> = world.get::<FieldPosition>(entity).cloned();
            let slot: Option<BoardSlot> = world.get::<BoardSlot>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(pos) = pos {
                        if let Some(slot) = slot {
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
