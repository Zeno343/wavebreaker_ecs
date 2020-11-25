use crate::{
    component::{
        Component,
        storage::Storage,
    },
    entity::Entity,
};

pub struct SparseVecStorage<C: Component + Clone> {
    storage: Vec<Option<C>>
}

impl<C: Component> Storage<C> for SparseVecStorage<C> {
    fn new() -> Self {
        Self {
            storage: Vec::new()
        }
    }

    fn insert(&mut self, entity: &Entity, component: C) {
        if entity.id >= self.storage.len() {
            self.storage.resize(entity.id + 1, None);
        }

        self.storage[entity.id] = Some(component);
    }

    fn read(&self, entity: &Entity) -> Option<&C> {
        match self.storage.get(entity.id) {
            Some(opt) => opt.as_ref(),
            None => None,
        }
    }

    fn write(&mut self, entity: &Entity) -> Option<&mut C> {
        match self.storage.get_mut(entity.id) {
            Some(opt) => opt.as_mut(),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        component::Component,
        entity::Entity,
    };
    use super::*;

    #[derive(Clone)]
    struct Color {
        name: String
    }
    
    impl Component for Color {
        type Storage = SparseVecStorage<Self>;
    }

    #[derive(Clone)]
    struct Name {
        name: String
    }
    
    impl Component for Name {
        type Storage = SparseVecStorage<Self>;
    }

    #[test]
    fn insert() {
        let mut colors = SparseVecStorage::new();

        let entity = Entity{ id: 0 };
        colors.insert(&entity, Color { name: "blue".to_string() });

        assert_eq!(colors.storage.len(), 1)
    }

    #[test]
    fn insert_non_contiguous() {
        let mut colors = SparseVecStorage::new();

        let entity1 = Entity{ id: 0 };
        let entity2 = Entity{ id: 4 };
        colors.insert(&entity1, Color { name: "blue".to_string() });
        colors.insert(&entity2, Color { name: "red".to_string() });

        assert_eq!(colors.storage.len(), 5)
    }
}
