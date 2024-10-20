mod common;
mod trigger;

pub use common::*;

use bevy::{
    ecs::system::SystemId,
    prelude::*,
    utils::HashMap,
};

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
pub struct Effects(Vec<EffectInstance>);

impl Effects {
    pub fn new(effects: Vec<EffectInstance>) -> Self {
        Self(effects)
    }

    pub fn get_effect(&self, index: usize) -> Option<&EffectInstance> {
        self.0.get(index)
    }
}

#[derive(Component)]
pub struct EffectTrigger<T: 'static + Send + Sync> {
    effect_idxs: Vec<usize>,
    _phantom_data: std::marker::PhantomData<T>,
}

pub trait Effect {
    fn activate(&self, commands: &mut Commands, self_entity: Entity) -> Vec<Box<dyn EffectAction>>;
    fn get_effect_speed(&self) -> i32;
}

#[derive(Clone, Copy)]
pub struct EffectId(pub usize);

pub struct EffectInstance {
    cooldown: usize,
    effect_id: EffectId,
}

impl EffectInstance {
    pub fn new(effect_id: EffectId) -> Self {
        Self {
            cooldown: 0,
            effect_id,
        }
    }
}

pub trait EffectAction {
    fn execute(&self, commands: &mut Commands, data: &EffectActionData);
}

pub struct EffectState {
    effect_index: usize,
    effect_data: EffectActionData,
}

pub struct EffectActionDestroy {
    targets_group: String,
}

impl EffectActionDestroy {
    pub fn new(targets_group: String) -> Self {
        Self { targets_group }
    }
}

impl EffectAction for EffectActionDestroy {
    fn execute(&self, commands: &mut Commands, data: &EffectActionData) {
        if let Some(targets) = data.targets.get(&self.targets_group) {
            for target in targets.iter() {
                commands.entity(*target).despawn_recursive();
            }
        } else {
            //TODO somehow fix this impossible state by cancelling this effect ig
            error!("impossible state");
        }
    }
}

pub struct EffectActionData {
    self_entity: Entity,
    action_index: u8,
    agent: Entity,
    targets: HashMap<String, Vec<Entity>>,
}

impl EffectActionData {
    pub fn new(
        self_entity: Entity,
        action_index: u8,
        agent: Entity,
        targets: HashMap<String, Vec<Entity>>,
    ) -> Self {
        Self {
            self_entity,
            action_index,
            agent,
            targets,
        }
    }
}

pub struct TargetGroup {
    name: String,
    data_query: QueryState<(), ()>,
}
