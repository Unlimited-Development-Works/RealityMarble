#[derive(Debug)]
enum ExitReason {
    Paused
}

pub struct EventProcessor;// {
    //EventLogConnection: Connection 
//}

impl EventProcessor {
    pub fn block(&self) -> Result<ExitReason, String> {
        Err("Not Implemented".to_string())
        //Ok(ExitReason::Paused)
    }
}

pub fn begin_event_system(path: Option<&str>) -> EventProcessor {
    /* let connection = match path {
        None => sqlite::open(":memory:").unwrap(),
        Some(path) => panic!("Not Implemented")
    };

    EventProcessor {
        EventLogConnection: connection 
    } */
    
    EventProcessor
}