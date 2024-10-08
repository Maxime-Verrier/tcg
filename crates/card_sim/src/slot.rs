use bevy::ecs::component::{ComponentHooks, StorageType};
pub use bevy::prelude::*;
use epithet::utils::LevelEntity;

use crate::{AgentOwned, Board, FieldPosition, OnBoard, OnField};

#[derive(Bundle)]
pub struct CardSlotBundle {
    pub card_slot: BoardSlot,
    pub transform: Transform,
    pub on_board: OnBoard,
    pub on_field: OnField,
    pub level_entity: LevelEntity,
    pub name: Name,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Default, Clone)]
pub struct BoardSlot(pub Option<Entity>);

impl Component for BoardSlot {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let pos = world.get::<FieldPosition>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(pos) = pos {
                        board.insert_by_slot(pos, entity);
                    }
                }
            }
        });
        hooks.on_remove(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let pos = world.get::<FieldPosition>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(pos) = pos {
                        board.remove_slot(&pos);
                    }
                }
            }
        });
    }
}
