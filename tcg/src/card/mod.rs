mod summon;

pub use summon::*;

use bevy::app::App;

use card_sim::BoardSlot;
use card_sim::OnSlot;

pub fn card_plugin(app: &mut App) {
    #[cfg(feature = "render")]
    {
        app.observe(on_slot_observer);
        app.observe(summon_action_execute); //TODO only attach to a target, on  self agent ?
        app.observe(summon_action_finish);
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
