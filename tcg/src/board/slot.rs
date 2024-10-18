use bevy::prelude::*;

pub(crate) fn create_slot_render(slot: In<Entity>, mut commands: Commands) {
    commands.entity(slot.0).insert(VisibilityBundle::default());
}
