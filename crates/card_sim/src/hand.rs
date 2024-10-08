use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

use crate::{AgentOwned, Board, Card, OnBoard, CARD_WIDTH};

#[derive(Event)]
pub struct OnCardAddedOnHand {
    pub entities: Vec<Entity>,
}

impl OnCardAddedOnHand {
    pub fn new(entities: Vec<Entity>) -> Self {
        Self { entities }
    }
}

#[derive(Clone, Copy)]
pub struct OnHand;

impl Component for OnHand {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let board_entity = world.get::<OnBoard>(entity).cloned();
            let player = world.get::<AgentOwned>(entity).cloned();

            if let Some(board_entity) = board_entity {
                if let Some(mut board) = world.get_mut::<Board>(board_entity.0) {
                    if let Some(player) = player {
                        board.insert_by_hand(player.0, entity);
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
                        board.remove_hand(player.0, &entity);
                    }
                }
            }
        });
    }
}

#[cfg(feature = "render")]
pub(crate) fn added_on_hand_observer(
    trigger: Trigger<OnAdd, OnHand>,
    mut commands: Commands,
    cards_on_hands: Query<(&OnBoard, &AgentOwned), (With<OnHand>, With<Card>)>,
    boards: Query<&Board>,
    cameras: Query<&Transform, With<Camera>>,
) {
    if let Ok((on_board, player)) = cards_on_hands.get(trigger.entity()) {
        if let Ok(board) = boards.get(on_board.0) {
            if let Some(hand) = board.get_by_hand(player.0) {
                let len = hand.len();
                let radius = CARD_WIDTH * 18.0;
                let angle_offset = -2.5; // degree
                let mut i = 0;
                let camera_pos = cameras.single().translation;
                let mut hand_pos = Transform::from_translation(cameras.single().forward() * 0.5);

                hand_pos.translation += Vec3::new(camera_pos.x, camera_pos.y, camera_pos.z);
                hand_pos.look_at(camera_pos, Vec3::Y);
                hand_pos.translation += Vec3::new(0.0, 0.0, 0.168);
                for card in hand.iter() {
                    let angle = angle_offset * (i as f32 - ((len - 1) as f32 / 2.0));
                    let x = (angle + 90.0).to_radians().cos() * radius;
                    let y = ((angle + 90.0).abs()).to_radians().sin() * radius - radius;

                    if let Some(mut card_entity) = commands.get_entity(*card) {
                        let mut transform = hand_pos;

                        transform.translation += hand_pos.right() * x + hand_pos.up() * y;
                        transform.translation += hand_pos.back() * 0.0001 * i as f32;
                        transform.rotation *= Quat::from_rotation_z(angle.to_radians());
                        card_entity.insert(TransformBundle {
                            local: transform,
                            ..default()
                        });
                        i += 1;
                    } else {
                        warn!(
                            "hand_observer got a non existent card entity from the board loookup"
                        );
                    }
                }
            }
        }
    }
}
