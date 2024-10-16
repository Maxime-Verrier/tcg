pub mod client_action;
mod hand;

use bevy_replicon::client::ClientSet;

#[cfg(feature = "render")]
#[cfg(feature = "client")]
use client_action::board_action_plugin;
pub use hand::*;

use card_sim::{
    packets::{player_join_packet_system, player_joined_packet_system},
    BoardSlot, OnSlot,
};

use crate::scene::on_board_agent_join;

pub(crate) fn board_plugin(app: &mut bevy::app::App) {
    #[cfg(feature = "render")]
    {
        #[cfg(feature = "client")]
        {
            board_action_plugin(app);
        }
        app.observe(remove_from_hand_observer);
        app.observe(on_slot_observer); //TODO  run if agent

        app.add_systems(
            Update,
            on_client_join_board_render.after(player_joined_packet_system), //TODO transform is on server side client to bruh
        );
    }
}

/// Place the summon entity on the field at the slot position
#[cfg(feature = "render")]
pub(crate) fn on_slot_observer(
    trigger: Trigger<OnInsert, OnSlot>,
    mut commands: Commands,
    on_slots: Query<&OnSlot>,
    slots: Query<&Transform, With<BoardSlot>>,
) {
    if let Ok(on_slot) = on_slots.get(trigger.entity()) {
        if let Ok(slot_transform) = slots.get(on_slot.0) {
            if let Some(mut entity) = commands.get_entity(trigger.entity()) {
                entity.insert(
                    Transform::from_translation(slot_transform.translation).with_rotation(
                        Quat::from_rotation_x(90.0_f32.to_radians())
                            * Quat::from_rotation_z(180.0_f32.to_radians()),
                    ),
                );
            }
        } else {
            error!("Could not change the summon entity to the slot position, the slot entity is not found, this should not be possible");
        }
    } else {
        error!("Could not change the summon entity to the slot position, the on_slot component was not found, this should not be possible");
    }
}
