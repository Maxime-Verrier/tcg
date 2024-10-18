use bevy::prelude::*;

use crate::{board, Board, OnBoard};

use super::Effects;

#[derive(Event)]
pub struct z<T> {
    event: T,
}

#[derive(Event)]
pub struct When<T> {
    event: T,
}

pub struct Summon;
