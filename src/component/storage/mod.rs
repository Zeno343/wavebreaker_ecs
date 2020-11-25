use std::any::Any;

use crate::{
    component::Component,
    Entity,
};

pub mod sparse_vec_storage;
pub use sparse_vec_storage::SparseVecStorage;

pub trait Storage<C: Component>: Any + Sized {
    fn new() -> Self;

    fn insert(&mut self, entity: &Entity, component: C);

    fn read(&self, entity: &Entity) -> Option<&C>;
    fn write(&mut self, entity: &Entity) -> Option<&mut C>;
}
