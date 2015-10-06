use super::events::{ EventStore };

/// A completely self contained machine for an operating function
pub struct Machine<'a> {
    events: Box<EventStore + 'a>
}

impl<'a> Machine<'a> {
    /// Create a new machine which will output events to the given event log
    pub fn new<T>(events: T) -> Machine<'a> where T : EventStore, T : 'a {    
        let m = Machine {
            events: Box::new(events)
        };
        
        //Setup bootstrap state in machine
        info!("Not implemented, machine bootstrap");
        
        m
    }
    
    /// Load a machine from the given event log
    pub fn rebuild<T>(events: T) -> Machine<'a> where T : EventStore, T : 'a {
        let mut m = Machine::new(events);
        
        let count = m.events.count();
        info!("Rebuilding Machine. Processing {:?} events", count);
        
        for i in 0 .. count {
            let event = { m.events.get(i) };
            match event {
                Some(evt) => m.apply(evt),
                None => panic!("Rebuilding machine failed to fetch event {:?}", i)
            }
        }
        
        m
    }
    
    /// Apply this machine (with all of it's state) to the given datum
    /// This mutates the machine into a new state
    pub fn apply(&mut self, data: Vec<u8>) {
    
        //Null event, has no effect
        if data.len() == 0 {
            return;
        }
    
        //Now process the data
        info!("Not implemented, process data");
        
        //Finally, put the new data into the event store (move into event store to save a copy)
        self.events.add(data);
    }
}