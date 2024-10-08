use action::{ActionFinishEvent, ActionInput, ActionState};
use bevy::ecs::system::SystemId;
pub use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Listener, On},
};
use card_sim::{BoardSlot, OnHand};

#[derive(Resource)]
pub struct SummonActionResource {
    pub execute_id: SystemId<ActionInput>,
    pub cancel_id: SystemId<ActionInput>,
    pub finish_id: SystemId<ActionInput>,
}

pub fn summon_action_execute(
    input: In<ActionInput>,
    mut commands: Commands,
    query: Query<Entity, With<BoardSlot>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let action_input = input.0;

    if action_input.entities.len() > 0 {
        let summon_entity = action_input.entities[0];

        for slot_entity in query.iter() {
            commands.entity(slot_entity).insert((
                On::<Pointer<Click>>::run(
                    |event: Listener<Pointer<Click>>,
                     mut commands: Commands,
                     actionners: Query<Entity, With<ActionState>>| {
                        commands.trigger_targets(ActionFinishEvent, actionners.get_single().unwrap());
                    },
                ),
            )).with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(0.1, 0.1, 0.1))),
                    ..default()
                });
            });
        }
        //TODO slot check to avoid soft lock
    } else {
        error!("No card entity to summon found in action input");
    }
}

pub fn summon_action_cancel(input: In<ActionInput>) {}

pub fn summon_action_finish(
    input: In<ActionInput>,
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<BoardSlot>>, mut meshes: ResMut<Assets<Mesh>>,
) {
    let action_input = input.0;

    if action_input.entities.len() > 0 {
        let summon_entity = action_input.entities[0];

        for (slot_entity, transform) in query.iter() {
            let mut slot_commands = commands.entity(slot_entity);

            slot_commands.despawn_descendants();
            println!("summon_entity: {:?}", summon_entity);
            if let Some(mut entity) = commands.get_entity(summon_entity) {
                entity.remove::<(On<Pointer<Click>>, OnHand)>();
                entity.insert(Transform {
                    translation: transform.translation,
                    rotation: (transform.rotation * Quat::from_rotation_x(90.0_f32.to_radians()) * Quat::from_rotation_z(180.0_f32.to_radians())),
                    scale: transform.scale,
                });
            }
        }
        //TODO slot check to avoid soft lock
    } else {
        error!("No card entity to summon found in action input");
    }
}
