pub trait EventStore {    
    /// Get the count of events in the store
    fn count(&self) -> u64;
    
    /// Get the event with the given sequence number
    fn get(&self, sequence: u64) -> Option<Vec<u8>>;
    
    /// Add a new event to the queue
    fn add(&mut self, event: Vec<u8>) -> u64;
}

pub struct InMemoryStore {
    events: Vec<Vec<u8>>
}

impl InMemoryStore {
    pub fn new() -> InMemoryStore {
        InMemoryStore {
            events: vec![]
        }
    }
}

impl EventStore for InMemoryStore {
    fn count(&self) -> u64 {
        self.events.len() as u64
    }
    
    fn get(&self, sequence: u64) -> Option<Vec<u8>> {
        match self.events.get(sequence as usize) {
            None => None,
            Some(_) => panic!("Not Implemented")
        }
    }
    
    fn add(&mut self, event: Vec<u8>) -> u64 {
        let seq = (self.events.len() as u64) + 1;
        self.events.push(event);
        
        seq
    }
}

#[test]
fn assert_that_creating_in_memory_store_creates_empty_store() {
    let store = InMemoryStore::new();
    assert!(store.count() == 0);
}

#[test]
fn assert_that_adding_to_in_memory_store_increments_count() {
    let mut store = InMemoryStore::new();
    store.add(vec![]);
    assert!(store.count() == 1);
}