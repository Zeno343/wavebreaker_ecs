#[allow(dead_code)]
pub mod component;
#[allow(dead_code)]
pub mod entity;
#[allow(dead_code)]
pub mod resource;

use component::ComponentManager;
use entity::EntityManager;
use resource::{
    Resource,
    ResourceManager,
};

pub struct World {
    pub components: ComponentManager,
    pub entities: EntityManager,
    pub resources: ResourceManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: ComponentManager::new(),
            entities: EntityManager::new(),
            resources: ResourceManager::new(),
        }
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.resources.insert(resource);
    }

    pub fn read_resource<R: Resource>(&mut self) -> &R {
        self.resources.read::<R>()
    }

    pub fn write_resource<R: Resource>(&mut self) -> &mut R {
        self.resources.write::<R>()
    }
}

#[cfg(test)]
mod tests {

}

