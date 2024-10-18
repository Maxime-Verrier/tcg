pub mod client_action;
mod hand;
mod slot;

use epithet::units::UnitPluginExt;

use card_sim::BoardSlot;

#[cfg(feature = "render")]
pub use cgf_board_mod_render::*;

pub(crate) fn board_plugin(app: &mut bevy::app::App) {
    app.add_unit::<BoardSlot>();

    #[cfg(feature = "render")]
    {
        use bevy::prelude::*;
        use card_sim::player_joined_packet_system;
        use client_action::board_action_plugin;
        use hand::on_client_join_board_render;
        use hand::remove_from_hand_observer;
        use slot::create_slot_render;

        #[cfg(feature = "client")]
        {
            board_action_plugin(app);
        }
        app.observe(remove_from_hand_observer);
        app.observe(on_slot_observer); //TODO  run if agent

        let id = app.register_system(create_slot_render);

        app.bind_render::<BoardSlot>(id);

        app.add_systems(
            Update,
            on_client_join_board_render.after(player_joined_packet_system), //TODO transform is on server side client to bruh
        );
    }
}

/// Place the summon entity on the field at the slot position
#[cfg(feature = "render")]
pub mod cgf_board_mod_render {
    use bevy::prelude::*;
    use card_sim::{BoardSlot, OnSlot};

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
}
