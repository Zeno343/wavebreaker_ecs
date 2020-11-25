use std::{
    any::{
        Any,
        TypeId,
    },
    collections::HashMap,
};

struct Entity {
    id: usize,
}

trait Component: Sized + 'static {
    type Storage: Storage<Self>;
}

trait Storage<C: Component>: Any + Sized {
    fn new() -> Self;

    fn insert(&mut self, entity: &Entity);
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

    fn get_storage_mut<C: Component>(&mut self) -> &mut <C as Component>::Storage {
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
mod tests {

}

