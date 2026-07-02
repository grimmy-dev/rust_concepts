// Arena owns single ownership for multiple values
// impl new, alloc, len, is_empty, get

pub struct Arena<T> {
    storage: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    pub fn alloc(&mut self, value: T) -> &mut T {
        self.storage.push(value);
        // safe to unwrap as we just pushed an item
        self.storage.last_mut().unwrap()
    }

    // Run time safety
    // pub fn alloc(&mut self, value: T) -> usize {
    //     self.storage.push(value);
    //      Return the index of value as ticket
    //      self.storage.len()-1
    // }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    // Get value with the ticket given when allocation
    // pub fn get(&self, ticket: usize) -> Option<&T> {
    //     self.storage.get(ticket)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // MUST NOT COMPILE
    fn it_works() {
        let mut arena = Arena::new();

        assert!(arena.is_empty());

        // Allocate "hello" and modify it.
        // This borrow (t1) must end before we mutable-borrow the arena again.
        let t1 = arena.alloc(String::from("hello"));

        // the second borrow will cause error and not compiles code and cause is can hold one mut ref at a time.
        let t2 = arena.alloc(String::from("world"));

        t1.push_str("!");

        assert_eq!(t2, "world");

        // Verify the final state of the arena
        assert_eq!(arena.len(), 2);
        assert_eq!(arena.is_empty(), false);

        // Use get() to retrieve items by their index (ticket)
        // assert_eq!(arena.get(0), Some(&String::from("hello!")));
        // assert_eq!(arena.get(1), Some(&String::from("world")));
        // assert_eq!(arena.get(2), None);
    }

    #[test]
    // MUST NOT COMPILE
    fn drop_arena_while_holding_reference() {
        let mut arena = Arena::new();

        // 1. `reference` borrows a value inside `arena`.
        // The lifetime of `reference` is now tied to the lifetime of `arena`.
        let reference = arena.alloc(String::from("hello"));

        // 2. We drop `arena`, destroying the underlying storage.
        drop(arena);

        // 3. We attempt to use the reference after the owner has been dropped.
        println!("{:?}", reference);
    }
}
