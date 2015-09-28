#[macro_use]
extern crate clap;
use clap::{ Arg, App, ArgGroup };

#[macro_use]
extern crate log;

mod events;
mod logging;
mod machine;

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
        ).arg(Arg::with_name("create")
            .help("If set, a new machine will be setup with default state")
            .short("c")
            .long("create")
        ).get_matches();

    //Are we setting up a new machine?
    let create = matches.is_present("create");
        
    // Create or load an event store
    let store = match matches.value_of("event_log") {
        Some(path) => {
            info!("Event Log Path: {}", path);
            panic!("Event persistent not implemented");
        },
        None => {
            warn!("No event log! All events are transient");
            events::InMemoryStore::new()
        }
    };
        
    // Create a machine
    let mut machine = machine::Machine::new(store);
    
    // If we're creating a machine, let's setup the default machine state now
    if (create) {
        warn!("Default machine state not implemented yet");
    }
        
    //Block until the machine terminates for some reason
    let result = machine.apply(machine::Noun::Atom(vec![]));

    println!("{:?}", result);
}