use std::{
    any::{
        Any,
        TypeId,
    },
    collections::HashMap,
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut,
    },
    sync::{
        RwLock,
        RwLockReadGuard,
        RwLockWriteGuard,
    },
};

pub trait Resource: Any { }

impl<A: Any> Resource for A { }

pub struct Write<'a, R: Resource> {
    inner: RwLockWriteGuard<'a, Box::<dyn Any>>,
    marker: PhantomData<R>
}

impl<R: Resource> Deref for Write<'_, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.inner
            .downcast_ref()
            .expect("Could not downcast resource")
    }
}

pub struct Read<'a, R: Resource> {
    inner: RwLockReadGuard<'a, Box::<dyn Any>>,
    marker: PhantomData<R>
}

impl<R: Resource> Deref for Read<'_, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.inner
            .downcast_ref()
            .expect("Could not downcast resource")
    }
}

impl<R: Resource> DerefMut for Write<'_, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
            .downcast_mut()
            .expect("Could not downcast resource")
    }
}

pub struct ResourceManager {
    storages: HashMap<TypeId, RwLock<Box<dyn Any>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    pub fn insert<R: Resource>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();

        self.storages.insert(type_id, RwLock::new(Box::new(resource)));
    }

    pub fn write<R: Resource>(&self) -> Write<R> {
        let type_id = TypeId::of::<R>();

        match self.storages.get(&type_id) {
            Some(probably_storage) => {
                Write::<R> {
                    inner: probably_storage
                        .write()
                        .expect("Could not access write lock"),
                    marker: PhantomData::<R>
                }
            },

            None => panic!("Could not access resource to write!"),
        }
    }

    pub fn read<R: Resource>(&self) -> Read<R> {
        let type_id = TypeId::of::<R>();

        match self.storages.get(&type_id) {
            Some(probably_storage) => {
                Read::<R> {
                    inner: probably_storage
                        .read()
                        .expect("Could not access write lock"),
                    marker: PhantomData::<R>
                }
                
            }
            None => panic!("Could not access resource to read!"),
        }
    }
}
