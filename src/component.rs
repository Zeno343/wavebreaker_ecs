use std::{
    any::{
        Any,
        TypeId,
    },
    collections::HashMap,
    ops::{
        Index,
        IndexMut,
    },
};

use crate::Entity;

trait Component: Clone + Sized + 'static {
    type Storage: Storage<Self>;
}

trait Storage<C: Component>: Any + Sized {
    fn new() -> Self;

    fn insert(&mut self, entity: &Entity, component: C);

    fn read(&self, entity: &Entity) -> Option<&C>;
    fn write(&mut self, entity: &Entity) -> Option<&mut C>;
}

struct SparseVecStorage<C: Component + Clone> {
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

struct ComponentManager {
    storages: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    fn write_storage<C: Component>(&mut self) -> &mut <C as Component>::Storage {
        let type_id = TypeId::of::<C>();

        if !self.storages.contains_key(&type_id) {
            let new_storage = <C as Component>::Storage::new();

            self.storages.insert(type_id, Box::new(new_storage));
        }

        match self.storages.get_mut(&type_id) {
            Some(probably_storage) => {
                match probably_storage.downcast_mut::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!(),
                }
            }
            None => unreachable!(),
        }
    }

    fn read_storage<C: Component>(&mut self) -> &<C as Component>::Storage {
        let type_id = TypeId::of::<C>();

        if !self.storages.contains_key(&type_id) {
            let new_storage = <C as Component>::Storage::new();

            self.storages.insert(type_id, Box::new(new_storage));
        }

        match self.storages.get_mut(&type_id) {
            Some(probably_storage) => {
                match probably_storage.downcast_mut::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!(),
                }
            }
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
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
    fn sparse_storage_insert() {
        let mut colors = SparseVecStorage::new();

        let entity = Entity{ id: 0 };
        colors.insert(&entity, Color { name: "blue".to_string() });

        assert_eq!(colors.storage.len(), 1)
    }

    #[test]
    fn sparse_storage_insert_non_contiguous() {
        let mut colors = SparseVecStorage::new();

        let entity1 = Entity{ id: 0 };
        let entity2 = Entity{ id: 4 };
        colors.insert(&entity1, Color { name: "blue".to_string() });
        colors.insert(&entity2, Color { name: "red".to_string() });

        assert_eq!(colors.storage.len(), 5)
    }

    #[test]
    fn multiple_read_storages() {
        let mut components = ComponentManager::new();

        let _colors = components.read_storage::<Color>();
        let _names = components.read_storage::<Name>();
    }

    #[test]
    fn multiple_write_storages() {
        let mut components = ComponentManager::new();

        let _colors = components.write_storage::<Color>();
        let _names = components.write_storage::<Name>();
    }
}
