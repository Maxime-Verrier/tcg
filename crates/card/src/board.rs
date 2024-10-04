use bevy::{ecs::component::{ComponentHooks, StorageType}, math::{IVec3, Vec2}, prelude::{Component, Entity}, utils::HashMap};
use std::collections::HashMap;

pub struct OnBoard(Entity);

#[derive(Component)]
pub struct OnField;

impl Component for OnBoard {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_remove(|mut world, card_entity, _component_id|{
            if let Some(on_field) = world.get::<OnBoard>(card_entity) {
				if let Some(mut field) = world.get_mut::<Board>(on_field.0) {
					field.remove_entity_from_field(&card_entity);
				}
            }
        });
    }
}


/// A component representing a board existing mainly for the purpose of lookup
/// This lookup component stay in sync with any entity having a OnField component
#[derive(Default, Debug)]
pub struct Board {
	pos_to_field_entity: HashMap<IVec3, Entity>,
	field_entity_to_pos: HashMap<Entity, IVec3>,
	player_to_field_entities: HashMap<u32, Vec<Entity>>, // New index for player entities
}

impl Board {
	pub fn add_entity_to_field(&mut self, pos: IVec3, entity: Entity, player: Player) {
		self.pos_to_field_entity.insert(pos, entity);
		self.field_entity_to_pos.insert(entity, pos);
		self.player_to_field_entities.entry(player).or_default().push(entity);
	}

	pub fn remove_entity_from_field(&mut self, entity: &Entity) -> Option<IVec3> {
		if let Some(pos) = self.field_entity_to_pos.remove(entity) {
			self.pos_to_field_entity.remove(&pos);
			for entities in self.player_to_field_entities.values_mut() {
				entities.retain(|&e| e != *entity);
			}
			return Some(pos);
		}
		None
	}

	pub fn get_field_entity(&self, pos: &IVec3) -> Option<&Entity> {
		self.pos_to_field_entity.get(pos)
	}

	pub fn get_card_by_entity(&self, entity: &Entity) -> Option<&IVec3> {
		self.field_entity_to_pos.get(entity)
	}

	pub fn query_by_player(&self, player: Player) -> Option<&Vec<Entity>> {
		self.player_to_field_entities.get(&player)
	}
}

impl Component for Board {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
		hooks.on_remove(|mut world, field_entity, _component_id| {
			if let Some(field) = world.get::<Board>(field_entity) {
				let card_entities: Vec<Entity> = field.pos_to_field_entity.values().cloned().collect();
				for card_entity in card_entities {
					if let Some(mut card_commands) = world.commands().get_entity(card_entity) {
						card_commands.remove::<OnBoard>();
					}
				}
			}
		});
	}
}