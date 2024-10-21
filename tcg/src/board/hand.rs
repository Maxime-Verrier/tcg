#[cfg(all(feature = "render", feature = "client"))]
pub(crate) use cfg_hand_client_render::*;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use card_sim::OnHand;

pub(crate) fn remove_from_hand_observer(
    trigger: Trigger<OnRemove, OnHand>,
    mut commands: Commands,
) {
    if let Some(mut entity_commands) = commands.get_entity(trigger.entity()) {
        //TODO just make an event and depending the place it will be executed ? or idk
        entity_commands.remove::<On<Pointer<Click>>>();
    }
}

#[cfg(all(feature = "render", feature = "client"))]
mod cfg_hand_client_render {
    use crate::board::client_action::{
        ClientAction, ClientActionState, ClientSummonAction, SummonActionFinishEvent,
    };
    use bevy::prelude::*;
    use bevy_mod_picking::prelude::*;
    use card_sim::{Board, ClientJoinedBoardPacket, OnBoard, CARD_WIDTH};

    pub(crate) fn on_client_join_board_render(
        mut packets: EventReader<ClientJoinedBoardPacket>,
        mut commands: Commands,
        boards: Query<&Board>,
        cameras: Query<&Transform, With<Camera>>,
    ) {
        for packet in packets.read() {
            if let Ok(board) = boards.get(packet.board) {
                let hands = board.cache.on_hand_lookup.iter();

                for (hand_agent, hand) in hands {
                    let client_own_flag = *hand_agent == packet.agent;

                    let len = hand.len();
                    let radius = CARD_WIDTH * 18.0;
                    let angle_offset = -2.5; // degree
                    let mut i = 0;
                    let camera_pos = cameras.single().translation;
                    let mut hand_pos =
                        Transform::from_translation(cameras.single().forward() * 0.5);

                    hand_pos.translation += Vec3::new(camera_pos.x, camera_pos.y, camera_pos.z);
                    hand_pos.look_at(camera_pos, Vec3::Y);
                    hand_pos.translation +=
                        Vec3::new(0.0, 0.0, if client_own_flag { 0.168 } else { -0.168 });
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

                            if client_own_flag {
                                card_entity.insert(On::<Pointer<Click>>::run(
                                    //TODO make On::Run ? bevy 0.15 soon to be released
                                    //TODO add self agent
                                    |event: Listener<Pointer<Click>>,
                                    mut commands: Commands,
                                    mut action_state: ResMut<ClientActionState>,
                                    on_boards: Query<&OnBoard>| {
                                        if let Ok(on_board) = on_boards.get(event.listener()) {
                                            action_state.execute_action(
                                                &mut commands,
                                                ClientAction::new(
                                                    Box::new(ClientSummonAction::new(
                                                        on_board.0,
                                                        event.listener(),
                                                    )),
                                                    Box::new(SummonActionFinishEvent),
                                                ),
                                            );
                                        }
                                    },
                                ));
                            }
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
}
