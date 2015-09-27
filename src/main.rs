#[macro_use]
extern crate clap;
use clap::{ Arg, App, ArgGroup };

#[macro_use]
extern crate log;

mod events;
mod logging;

fn main() {
    //Setup logging
    logging::PrintLogger::init().unwrap();
    info!("{0} v{1}", "Starting Reality Marble ", env!("CARGO_PKG_VERSION"));

    //parse command line args
    let matches = App::new("Reality Marble")
        .version(&crate_version!()[..])
        .author("Martin Evans <martindevans@gmail.com>")
        .about("Computing From The Future")
        .arg(Arg::with_name("event_log")
            .help("Sets the path to the event log directory to load or create")
            .required(true)
            .takes_value(true)
            .short("e")
            .long("event_log")
        ).arg(Arg::with_name("in_memory")
            .help("Indicates that the event log will be transient (in memory only)")
            .short("m")
            .long("in_memory")
        ).arg_group(ArgGroup::with_name("log")
            .required(true)
            .add_all(&["event_log", "in_memory"])
        ).get_matches();

    let log_path = matches.value_of("event_log");
    match log_path {
        Some(path) => info!("Event Log Path: {}", path),
        None => warn!("No event log! All events are transient")
    }
        
    //Setup the event system
    let events = events::begin_event_system(log_path);
    
    //Block until the event system terminates
    let result = events.block();
    
    //There are two reasons for the event system to stop processing...
    match result {
        // This was a deliberate termination, i.e. universe is being paused
        Ok(code) => info!("The Machine Stops (Reason): {:?}", code),
        
        // Something went wrong, i.e. universe has been destroyed
        Err(msg) => panic!(msg)
    }
}