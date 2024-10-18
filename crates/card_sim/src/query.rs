use bevy::prelude::*;
use bevy::{
    ecs::{
        component::ComponentId,
        query::{QueryData, QueryFilter},
    },
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct TagRegistry {
    tags: HashMap<String, ComponentId>,
}

pub trait RuntimeQueryExt {
    fn query_runtime<D: QueryData, F: QueryFilter>(
        &mut self,
        query_data: &[RunetimeQueryId],
    ) -> QueryBuilder<D, F>;

    fn query_runtime_tag<D: QueryData, F: QueryFilter>(
        &mut self,
        query_tags: &[RuntimeQueryTag],
    ) -> QueryBuilder<D, F>;
}

impl RuntimeQueryExt for World {
    fn query_runtime<D: QueryData, F: QueryFilter>(
        &mut self,
        query_data: &[RunetimeQueryId],
    ) -> QueryBuilder<D, F> {
        let mut builder: QueryBuilder<D, F> = QueryBuilder::<D, F>::new(self);

        for data in query_data.iter() {
            data.build(&mut builder);
        }

        builder
    }

    fn query_runtime_tag<D: QueryData, F: QueryFilter>(
        &mut self,
        query_tags: &[RuntimeQueryTag],
    ) -> QueryBuilder<D, F> {
        let mut ids = Vec::new();
        let tag_registry = self.get_resource::<TagRegistry>().unwrap();

        for data in query_tags.iter() {
            if let Ok(query_data) = data.to_ids(tag_registry) {
                ids.push(query_data);
            }
        }

        self.query_runtime::<D, F>(&ids)
    }
}

#[derive(Serialize, Deserialize)]
pub enum RuntimeQueryData<T> {
    With(Vec<T>),
    Without(Vec<T>),
    And(Vec<RuntimeQueryData<T>>),
    Or(Vec<RuntimeQueryData<T>>),
}

pub type RuntimeQueryTag = RuntimeQueryData<String>;
pub type RunetimeQueryId = RuntimeQueryData<ComponentId>;

impl RunetimeQueryId {
    pub fn build<D: QueryData, F: QueryFilter>(&self, builder: &mut QueryBuilder<D, F>) {
        match self {
            RuntimeQueryData::With(data) => {
                for d in data.iter() {
                    builder.with_id(*d);
                }
            }
            RuntimeQueryData::Without(data) => {
                for d in data.iter() {
                    builder.without_id(*d);
                }
            }
            RuntimeQueryData::And(data) => {
                for d in data.iter() {
                    builder.and(|builder| d.build(builder));
                }
            }
            RuntimeQueryData::Or(data) => {
                for d in data.iter() {
                    builder.or(|builder| d.build(builder));
                }
            }
        }
    }
}

impl RuntimeQueryTag {
    pub fn to_ids(&self, tag_registry: &TagRegistry) -> Result<RunetimeQueryId, String> {
        match self {
            RuntimeQueryData::With(data) => {
                let ids: Result<Vec<_>, _> = data.iter()
                    .map(|d| tag_registry.tags.get(d)
                        .ok_or_else(|| format!("Warning: Component '{}' not found in TagRegistry during RuntimeQueryData::to_ids", d)).copied())
                    .collect();
                Ok(RuntimeQueryData::With(ids?))
            }
            RuntimeQueryData::Without(data) => {
                let ids: Result<Vec<_>, _> = data.iter()
                    .map(|d| tag_registry.tags.get(d)
                        .ok_or_else(|| format!("Warning: Component '{}' not found in TagRegistry during RuntimeQueryData::to_ids", d)).copied())
                    .collect();
                Ok(RuntimeQueryData::Without(ids?))
            }
            RuntimeQueryData::And(data) => {
                let ids: Result<Vec<_>, _> = data.iter().map(|d| d.to_ids(tag_registry)).collect();
                Ok(RuntimeQueryData::And(ids?))
            }
            RuntimeQueryData::Or(data) => {
                let ids: Result<Vec<_>, _> = data.iter().map(|d| d.to_ids(tag_registry)).collect();
                Ok(RuntimeQueryData::Or(ids?))
            }
        }
    }
}
