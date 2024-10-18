mod common;
mod trigger;

pub use common::*;

use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};

pub(crate) fn effect_plugin(app: &mut App) {}

#[derive(Resource)]
pub struct EffectRegistry {
    effects: HashMap<String, SystemId>,
}

impl Default for EffectRegistry {
    fn default() -> Self {
        Self::new()
    }
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

pub fn setup_effects(effect_registry: ResMut<EffectRegistry>) {
    //    effect_registry.register_effect("destroy".to_string(), SystemId::new());
}

//TODO comments about invariants effects
#[derive(Component, Default)]
pub struct Effects(Vec<Box<dyn Effect + Send + Sync>>);

impl Effects {
    pub fn new(effects: Vec<Box<dyn Effect + Send + Sync>>) -> Self {
        Self(effects)
    }

    pub fn get_effect(&self, index: usize) -> Option<&Box<dyn Effect + Send + Sync>> {
        self.0.get(index)
    }
}

#[derive(Component)]
pub struct EffectTrigger<T: 'static + Send + Sync> {
    effect_idxs: Vec<usize>,
    _phantom_data: std::marker::PhantomData<T>,
}

pub trait Effect {
    fn activate(&self, world: &mut World, entity: Entity);
    fn get_effect_spped(&self) -> i32;
    fn instance(&self) -> Box<dyn Effect + 'static + Send + Sync>;
}

pub struct EffectId(pub usize);

pub struct EffectSystem {
    system_id: SystemId,
    effect_speed: i32,
}

impl EffectSystem {
    pub fn new(system_id: SystemId, effect_speed: i32) -> Self {
        Self {
            system_id,
            effect_speed,
        }
    }
}

impl Effect for EffectSystem {
    fn activate(&self, world: &mut World, entity: Entity) {
        world.run_system(self.system_id);
    }

    fn get_effect_spped(&self) -> i32 {
        todo!()
    }

    fn instance(&self) -> Box<dyn Effect + 'static + Send + Sync> {
        Box::new(EffectSystem::new(self.system_id, self.effect_speed))
    }
}

pub struct TargetGroup {
    name: String,
    data_query: QueryState<(), ()>,
}
