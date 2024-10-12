use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{AgentOwned, Board, OnBoard};

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
                        board.insert_on_hand(player.0, entity);
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
                        board.remove_from_hand(player.0, &entity);
                    }
                }
            }
        });
    }
}
