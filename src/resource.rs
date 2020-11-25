use std::{
    any::{
        Any,
        TypeId,
    },
    collections::HashMap,
};

pub trait Resource: Any + 'static { }

impl<A: Any + 'static> Resource for A { }

pub struct ResourceManager {
    storages: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    pub fn insert<R: Resource>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();

        self.storages.insert(type_id, Box::new(resource));
    }

    pub fn write<R: Resource>(&mut self) -> &mut R {
        let type_id = TypeId::of::<R>();

        match self.storages.get_mut(&type_id) {
            Some(probably_storage) => {
                match probably_storage.downcast_mut::<R>() {
                    Some(storage) => storage,
                    None => panic!(),
                }
            }
            None => panic!(),
        }
    }

    pub fn read<R: Resource>(&mut self) -> &R {
        let type_id = TypeId::of::<R>();

        match self.storages.get_mut(&type_id) {
            Some(probably_storage) => {
                match probably_storage.downcast_ref::<R>() {
                    Some(storage) => storage,
                    None => panic!(),
                }
            }
            None => panic!(),
        }
    }
}
