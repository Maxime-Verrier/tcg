use bevy::prelude::*;
use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        entity::MapEntities,
    },
    utils::HashMap,
};
use epithet::utils::LevelEntity;
use serde::{Deserialize, Serialize};

use crate::{Board, OnBoard, OnField};

use super::BoardLookup;

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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct BoardSlot(pub IVec3, pub Option<Entity>);

impl Component for BoardSlot {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            if let (Some(board_entity), Some(slot)) = (
                world.get::<OnBoard>(entity).cloned(),
                world.get::<BoardSlot>(entity).cloned(),
            ) {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.lookup.insert_slot(slot.0, entity);
                }
            }
        });

        hooks.on_remove(|mut world, entity, _component_id| {
            if let (Some(board_entity), Some(slot)) = (
                world.get::<OnBoard>(entity).cloned(),
                world.get::<BoardSlot>(entity).cloned(),
            ) {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    board.lookup.remove_slot(&slot.0);
                    board.lookup.remove_from_slot(&slot.0); //TODO system that check invalid place state (multiple) and return to hand if the case or no place
                }
                if let Some(entity_in_slot) = slot.1 {
                    if let Some(mut entity_in_slot_commands) =
                        world.commands().get_entity(entity_in_slot)
                    {
                        entity_in_slot_commands.remove::<OnSlot>();
                    }
                }
            }
        });
    }
}

impl MapEntities for BoardSlot {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.1 = self.1.map(|entity| entity_mapper.map_entity(entity));
    }
}

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub struct OnSlot(pub Entity);

impl Component for OnSlot {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            if let (Some(on_board), Some(on_slot), Some(slot)) = (
                world.get::<OnBoard>(entity).cloned(), // TODO get mut when there will no need to clone anymore with future bevy update
                world.get::<OnSlot>(entity).cloned(),
                world.get::<BoardSlot>(entity).cloned(),
            ) {
                if let Some(mut board) = world.get_mut::<Board>(on_board.0) {
                    board.lookup.insert_on_slot(slot.0, entity);
                }

                // Check if there is already an entity in the slot which should not be possible except if there was no verification before inserting it
                // Will go in a invalid place state and will get cleanup and returned to the hand
                if let Some(old_entity) = slot.1 {
                    error!("OnSlot component inserted pointing to a slot entity that already has an entity in it (old entity: {:?}, new_entity {:?}), this is a code error that will cause a invalid place error, check the verification before inserting OnSlot", old_entity, entity);
                    if let Some(mut old_entity_commands) = world.commands().get_entity(old_entity) {
                        old_entity_commands.remove::<OnSlot>();
                    }
                }
                world.get_mut::<BoardSlot>(entity).unwrap().1 = Some(entity); //TODO modify in future bevy update so there is no need to get it again
            }
        });

        hooks.on_remove(|mut world, entity, _component_id| {
            if let (Some(on_board), Some(on_slot), Some(slot)) = (
                world.get::<OnBoard>(entity).cloned(),
                world.get::<OnSlot>(entity).cloned(),
                world.get::<BoardSlot>(entity).cloned(),
            ) {
                if let Some(mut board) = world.get_mut::<Board>(on_board.0) {
                    board.lookup.remove_from_slot(&slot.0);
                    world.get_mut::<BoardSlot>(entity).unwrap().1 = None; //TODO modify in future bevy update so there is no need to get it again
                }
            }
        });
    }
}

impl MapEntities for OnSlot {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.0 = entity_mapper.map_entity(self.0);
    }
}

impl BoardLookup {
    pub(crate) fn insert_slot(&mut self, pos: IVec3, entity: Entity) {
        self.slots_lookup.insert(pos, entity);
    }

    pub fn insert_on_slot(&mut self, pos: IVec3, entity: Entity) {
        self.on_slot_lookup.insert(pos, entity);
    }

    pub(crate) fn remove_from_slot(&mut self, pos: &IVec3) -> Option<Entity> {
        self.on_slot_lookup.remove(pos)
    }

    pub(crate) fn remove_slot(&mut self, pos: &IVec3) -> Option<Entity> {
        self.slots_lookup.remove(pos)
    }

    pub fn get_on_slot(&self, pos: &IVec3) -> Option<&Entity> {
        self.slots_lookup.get(pos)
    }

    pub fn get_slots(&self) -> &HashMap<IVec3, Entity> {
        &self.slots_lookup
    }
}
