pub use bevy::prelude::*;
use card_sim::{AgentOwned, Board, Card, OnHand, OnBoard, CARD_WIDTH};

#[cfg(feature = "render")]
pub(crate) fn inserted_on_hand_observer(
    trigger: Trigger<OnInsert, OnHand>,
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