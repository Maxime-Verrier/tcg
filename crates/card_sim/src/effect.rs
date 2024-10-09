use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct EffectEvent(pub Entity);

pub struct Effect;
