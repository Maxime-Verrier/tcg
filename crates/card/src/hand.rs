use bevy::{ecs::component::{ComponentHooks, StorageType}, prelude::*, utils::warn};

use crate::{Board, Card, OnBoard, Player};

#[derive(Clone, Copy)]
pub struct OnHand;

impl Component for OnHand {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
			let board_entity = world.get::<OnBoard>(entity).cloned();
			let player = world.get::<Player>(entity).cloned();

			if let Some(board_entity) = board_entity {
				if let Some(mut board) = world.get_mut::<Board>(board_entity.0){
					if let Some(player) = player {
						board.insert_by_hand(player.0, entity);
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
						board.remove_hand(player.0, &entity);
					}
				}
			}
		});
	}
}

pub(crate) fn added_on_hand_observer(trigger: Trigger<OnInsert, OnHand>, mut commands: Commands, cards_on_hands: Query<(&OnBoard, &Player), (With<OnHand>, With<Card>)>, boards: Query<&Board>) {
    if let Ok((on_board, player)) = cards_on_hands.get(trigger.entity()) {
        if let Ok(board) = boards.get(on_board.0) {
            if let Some(hand) = board.get_by_hand(player.0) {
                let len = hand.len();
                let mut i = 0;
                println!("hand_observer got a hand with {} cards", len);
                for card in hand.iter() {
                    println!("hand_observer got a card entity from the board loookup");
                    if let Some(mut card_entity) = commands.get_entity(*card) {
                        println!("hand_observer got a card entity from the board loookup");
                        card_entity.insert(TransformBundle {
                            local: Transform::from_translation(Vec3::new(0.1 * i as f32, 0.0, 0.0)),
                            ..default()
                        });
                        i = i + 1;
                    }
                    else {
                        warn!("hand_observer got a non existent card entity from the board loookup");
                    }
                }
            }
        }
    }
}