#[derive(Clone, Copy, Debug)]
pub struct Entity {
    pub id: usize,
}

pub struct EntityManager {
    current_id: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            current_id: 0
        }
    }

    pub fn next_entity(&mut self) -> Entity {
        let entity = Entity {
            id: self.current_id
        };

        self.current_id += 1;

        entity
    }
}


