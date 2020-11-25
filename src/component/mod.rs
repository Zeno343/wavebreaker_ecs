use std::{
    any::{
        Any,
        TypeId,
    },
    collections::HashMap,
};

pub use crate::entity::Entity;

pub mod storage;
use storage::Storage;

pub trait Component: Clone + Sized + 'static {
    type Storage: Storage<Self>;
}

pub struct ComponentManager {
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
                    None => panic!(),
                }
            }
            None => panic!(),
        }
    }

    fn read_storage<C: Component>(&mut self) -> &<C as Component>::Storage {
        let type_id = TypeId::of::<C>();

        if !self.storages.contains_key(&type_id) {
            let new_storage = <C as Component>::Storage::new();

            self.storages.insert(type_id, Box::new(new_storage));
        }

        match self.storages.get(&type_id) {
            Some(probably_storage) => {
                match probably_storage.downcast_ref::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => panic!(),
                }
            }
            None => panic!(),
        }
    }

    fn get<C: Component>(&mut self, entity: &Entity) -> Option<&C> {
        let storage = self.read_storage::<C>();

        storage.read(entity)
    }

    fn get_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
        let storage = self.write_storage::<C>();

        storage.write(entity)
    }
}

#[cfg(test)]
mod test {
    use crate::component::{
        Component,
        ComponentManager,
        storage::SparseVecStorage,
    };

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
