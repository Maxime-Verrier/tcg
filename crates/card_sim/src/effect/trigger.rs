use bevy::{
    ecs::{system::SystemState, world::Command},
    prelude::*,
};

use crate::{Board, OnBoard};

use super::Effects;

#[derive(Component)]
pub struct EffectTrigger<T: Event> {
    effect_idxs: Vec<usize>,
    _phantom_data: std::marker::PhantomData<T>,
}

pub struct TriggerEffectsCommand<T: 'static + Send + Sync> {
    board: Entity,
    phantom: std::marker::PhantomData<T>,
}

impl<T: 'static + Send + Sync> TriggerEffectsCommand<T> {
    pub fn new(board: Entity) -> Self {
        Self {
            board,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Event + 'static + Send + Sync> Command for TriggerEffectsCommand<T> {
    fn apply(self, world: &mut World) {
        let mut system_state =
            SystemState::<(Query<(&EffectTrigger<T>, &Effects)>, Query<&mut Board>)>::new(world);
        let (triggers, mut boards) = system_state.get_mut(world);

        if let Ok(mut board) = boards.get_mut(self.board) {
            for entity in board
                .cache
                .get_entities()
                .iter()
                .cloned()
                .collect::<Vec<_>>()
            {
                if let Ok((trigger, effects)) = triggers.get(entity) {
                    //TODO invariants ? make the match a draw or cancel the effect
                    for idx in trigger.effect_idxs.iter() {
                        if let Some(effect) = effects.get_effect(*idx) {
                            board.trigger_effect(entity, *idx);
                        } else {
                            warn!("EffectTriggerCommand: effects indexs are broken, this should not be possible, skipping");
                        }
                    }
                }
            }
        } else {
            warn!("EffectTriggerCommand's entity has no board, skipping");
        }
    }
}
