use super::events::{ EventStore };

#[derive(Debug)]
pub enum Noun {
    Atom(Vec<u8>),
    
    Cell(Box<Noun>, Box<Noun>)
}

/// A completely self contained machine for an operating function
pub struct Machine<'a> {
    events: Box<EventStore + 'a>
}

impl<'a> Machine<'a> {
    /// Create a new machine which will output events to the given event log
    pub fn new<T>(events: T) -> Machine<'a> where T : EventStore, T : 'a {
        Machine {
            events: Box::new(events)
        }
    }
    
    /// Apply this machine (with all of it's state) to the given datum
    /// This produces a new noun (effectively a result) as well as mutates the machine state
    pub fn apply(&mut self, data: Noun) -> Noun {
    
        //First, put the new data into the event store
        self.events.add(data);
    
        //Now process the data
        info!("Not implemented, process data");
        Noun::Atom(vec![])
    }
}