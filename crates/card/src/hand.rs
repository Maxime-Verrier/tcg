use bevy::{ecs::component::{ComponentHooks, StorageType}, prelude::*, utils::warn};

use crate::{Board, Card, OnBoard, Player, CARD_WIDTH};

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

                let radius = CARD_WIDTH * 7.0;
                let angle_offset = -10.0; // degree
                let mut i = 0;

                for card in hand.iter() {
                    let angle = angle_offset * (i as f32 - ((len - 1) as f32 / 2.0));
                    let x = (angle + 90.0).to_radians().cos() * radius;
                    let z = ((angle + 90.0).abs()).to_radians().sin() * -radius + radius;

                    if let Some(mut card_entity) = commands.get_entity(*card) {
                        card_entity.insert(TransformBundle {
                            local: Transform::from_translation(Vec3::new(x, 0.0001 * i as f32, z)).with_rotation(Quat::from_rotation_y(angle.to_radians())),
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