use std::hash::Hash;

use bevy::{
    ecs::{component::{ComponentHooks, StorageType}, system::SystemState},
    math::{IVec3, Vec2},
    prelude::{Component, Entity, Query},
    utils::{tracing::field::Field, HashMap, HashSet},
};

/// A component representing a board existing both as a marker and a lookup table to get entity on the board by common values
/// The reason for this lookup table exist is to reduce iteration when needing to get entities by x by value as we can't query entites just for x board/hand/player/etc..
/// ex: entities on this board, entities at x pos of the board, cards of x player etc
/// The lookup tables are automaticly synched internally when entites get their component removed/changed/added so no need to their values to the table
#[derive(Default, Debug)]
pub struct Board {
    // Lookup maps
    pos_lookup: HashMap<FieldPosition, Entity>,
	hand_lookup : HashMap<u32, HashSet<Entity>>,

    // List of entity on the board and a player lookup at the same time
    // Not using a Vec with no associate value as it's will be mean reallocating everything and iteration are rarer than insert/remove, even if it's doesnt rly matter at the frequency of a card game
    board_lookup: HashSet<Entity>,
    player_lookup: HashMap<u32, HashSet<Entity>>,
}

impl Board {
    fn insert_on_board(&mut self, entity: Entity) {
        self.board_lookup.insert(entity);
    }

    fn insert_by_player(&mut self, player: u32, entity: Entity) {
		self.player_lookup.entry(player).or_insert_with(HashSet::new).insert(entity);

    }

    fn insert_by_pos(&mut self, pos: FieldPosition, entity: Entity) {
        self.pos_lookup.insert(pos, entity);
    }

    pub(crate) fn insert_by_hand(&mut self, player: u32, entity: Entity) {
		self.hand_lookup.entry(player).or_insert_with(HashSet::new).insert(entity);
    }

    fn remove_from_board(&mut self, entity: &Entity) -> bool {
        self.board_lookup.remove(entity)
    }

    fn remove_pos(&mut self, pos: &FieldPosition) -> Option<Entity> {
        self.pos_lookup.remove(pos)
    }

    fn remove_player(&mut self, player: u32, entity: &Entity) -> bool {
        self.player_lookup
            .get_mut(&player)
            .map_or(false, |entities| entities.remove(entity))
    }

    pub(crate) fn remove_hand(&mut self, player: u32, entity: &Entity) -> bool {
        self.hand_lookup
            .get_mut(&player)
            .map_or(false, |entities| entities.remove(entity))
    }

    pub fn get_by_pos(&self, pos: &FieldPosition) -> Option<&Entity> {
        self.pos_lookup.get(pos)
    }

    pub fn get_by_player(&self, player: u32) -> Option<&HashSet<Entity>> {
        self.player_lookup.get(&player)
    }

	pub fn get_by_hand(&self, player: u32) -> Option<&HashSet<Entity>> {
		self.hand_lookup.get(&player)
	}
}

impl Component for Board {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, field_entity, _component_id| {
            if let Some(field) = world.get::<Board>(field_entity) {
                let card_entities: Vec<Entity> = field.board_lookup.iter().cloned().collect();

                for card_entity in card_entities {
                    if let Some(mut card_commands) = world.commands().get_entity(card_entity) {
                        card_commands.remove::<OnBoard>();
                        card_commands.remove::<OnField>();
                        card_commands.remove::<FieldPosition>();
                    }
                }
            }
        });
    }
}

#[derive(Clone, Copy)]
pub struct Player(pub u32);

impl Component for Player {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
			let board_entity = world.get::<OnBoard>(entity).cloned();
			let player = world.get::<Player>(entity).cloned();

			if let Some(board_entity) = board_entity {
				if let Some(mut board) = world.get_mut::<Board>(board_entity.0){
					if let Some(player) = player {
						board.insert_by_player(player.0, entity);
					}
				}
			}
		});
        hooks.on_remove(|mut world, entity, _component_id| {
			let board_entity = world.get::<OnBoard>(entity).cloned();
			let player = world.get::<Player>(entity).cloned();

			if let Some(board_entity) = board_entity {
				if let Some(mut board) = world.get_mut::<Board>(board_entity.0){
					if let Some(player) = player {
						board.remove_player(player.0, &entity);
					}
				}
			}
		});
	}
}

/// Mark the entity as part of x board
/// TODO speak of sync/parameters
#[derive(Clone, Copy)]
pub struct OnBoard(pub Entity);

impl Component for OnBoard {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
			let board_entity = world.get::<OnBoard>(entity).cloned();
			let player = world.get::<Player>(entity).cloned();
			let pos = world.get::<FieldPosition>(entity).cloned();

			if let Some(board_entity) = board_entity {
				if let Some(mut board) = world.get_mut::<Board>(board_entity.0){
					board.insert_on_board(entity);

					if let Some(player) = player {
						board.insert_by_player(player.0, entity);
					}
					if let Some(pos) = pos {
						board.insert_by_pos(pos, entity);
					}
				}
			}
		});
        hooks.on_remove(|mut world, entity, _component_id| {
			let board_entity = world.get::<OnBoard>(entity).cloned();
			let player = world.get::<Player>(entity).cloned();
			let pos = world.get::<FieldPosition>(entity).cloned();

			if let Some(board_entity) = board_entity {
				if let Some(mut board) = world.get_mut::<Board>(board_entity.0){
					board.remove_from_board(&entity);

					if let Some(player) = player {
						board.remove_player(player.0, &entity);
					}
					if let Some(pos) = pos {
						board.remove_pos(&pos);
					}
				}
			}
        });
    }
}

#[derive(Component)]
pub struct OnField;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldPosition(pub IVec3);

impl Component for FieldPosition {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
			let board_entity = world.get::<OnBoard>(entity).cloned();
			let pos = world.get::<FieldPosition>(entity).cloned();

			if let Some(board_entity) = board_entity {
				if let Some(mut board) = world.get_mut::<Board>(board_entity.0){
					if let Some(pos) = pos {
						board.insert_by_pos(pos, entity);
					}
				}
			}
		});
        hooks.on_remove(|mut world, entity, _component_id| {
			let board_entity = world.get::<OnBoard>(entity).cloned();
			let player = world.get::<FieldPosition>(entity).cloned();

			if let Some(board_entity) = board_entity {
				if let Some(mut board) = world.get_mut::<Board>(board_entity.0){
					if let Some(pos) = player {
						board.remove_pos(&pos);
					}
				}
			}
		});
	}
}
