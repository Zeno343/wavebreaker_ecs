#[allow(dead_code)]
pub mod component;
#[allow(dead_code)]
pub mod entity;
#[allow(dead_code)]
pub mod resource;

use component::ComponentManager;
use entity::EntityManager;
use resource::ResourceManager;

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
}

#[cfg(test)]
mod tests {

}

