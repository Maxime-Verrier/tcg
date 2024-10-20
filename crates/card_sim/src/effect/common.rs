use bevy::prelude::*;



#[derive(Event)]
pub struct z<T> {
    event: T,
}

#[derive(Event)]
pub struct When<T> {
    event: T,
}

pub struct Summon;
