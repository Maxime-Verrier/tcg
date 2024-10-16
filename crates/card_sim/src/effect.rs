use std::error::Error;

use bevy::{
    ecs::{
        component::{self, ComponentId},
        query::{self, QueryData, QueryFilter, WorldQuery},
        system::{SystemId, SystemParam},
    },
    prelude::*,
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

use crate::{Board, CardId, OnBoard};

#[derive(Event)]
pub struct EffectEvent(pub Entity);

#[derive(Resource)]
pub struct EffectRegistry {
    effects: HashMap<String, SystemId>,
}

impl EffectRegistry {
    pub fn new() -> Self {
        Self {
            effects: HashMap::new(),
        }
    }

    pub fn register_effect(&mut self, name: String, effect: SystemId) {
        self.effects.insert(name, effect);
    }

    pub fn get_effect(&self, name: &str) -> Option<&SystemId> {
        self.effects.get(name)
    }
}

pub fn setup_effects(mut effect_registry: ResMut<EffectRegistry>) {
    //    effect_registry.register_effect("destroy".to_string(), SystemId::new());
}

pub trait EffectGroup {
    fn activate(&self, commands: &mut Commands, agent: Entity);
    fn clone_group(&self) -> Box<dyn EffectGroup + 'static + Send + Sync>;
}

#[derive(Component, Default)]
pub struct Effects(pub Vec<Box<dyn EffectGroup + Send + Sync>>);

pub struct Trigger<T> {
    effects: Vec<usize>,
    _phantom_data: std::marker::PhantomData<T>,
}

pub trait Effect {
    fn activate(&self, world: &mut World, entity: Entity, instance: &EffectInstance);
}

pub struct EffectId(pub usize);

pub struct EffectInstance {
    cooldown: usize,
    effect_id: EffectId
}

pub struct EffectSystem {
    system_id: SystemId,
}

impl EffectSystem {
    pub fn new(system_id: SystemId) -> Self {
        Self {
            system_id
        }
    }
}

impl Effect for EffectSystem {
    fn activate(&self, world: &mut World, entity: Entity, instance: &EffectInstance) {
        world.run_system(self.system_id);
    }
}

pub struct TargetGroup {
    name: String,
    data_query: QueryState<(), ()>,
}