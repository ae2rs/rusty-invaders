#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityUid(u64);

#[derive(Default)]
pub struct UidGenerator {
    next: u64,
}

impl UidGenerator {
    pub fn generate(&mut self) -> EntityUid {
        let id = EntityUid(self.next);

        self.next = self.next.checked_add(1).expect("ran out of entity IDs");

        id
    }
}
