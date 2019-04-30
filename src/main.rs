use std::error::Error;
use std::sync::mpsc;

use log::{info, error};
use neovim_lib::neovim::Neovim;
use neovim_lib::session::Session;
use simplelog::{Config, Level, LevelFilter, WriteLogger};

pub mod event;
pub mod event_handlers;
pub mod handler;

use crate::event::Event;
use crate::event_handlers::search::search;
use crate::handler::NeovimHandler;


fn main() {
    use std::process;

    init_logging().expect("scorched earth: unable to initialize logger.");

    match start_program() {
        Ok(_) => {
            info!("exiting");
            process::exit(0);
        }
        Err(msg) => {
            error!("{}", msg);
            process::exit(1);
        }
    };
}


fn init_logging() -> Result<(), Box<Error>> {
    use std::env;
    use std::fs::File;

    let _log_level_filter =
        match env::var("LOG_LEVEL").unwrap_or(String::from("trace")).to_lowercase().as_ref() {
            "debug" => LevelFilter::Debug,
            "error" => LevelFilter::Error,
            "info" => LevelFilter::Info,
            "off" => LevelFilter::Off,
            "trace" => LevelFilter::Trace,
            "warn" => LevelFilter::Warn,
            _ => LevelFilter::Off,
        };

    let log_level_filter = LevelFilter::Debug;

    let config = Config {
        time: Some(Level::Error),
        level: Some(Level::Error),
        target: Some(Level::Error),
        location: Some(Level::Error),
        time_format:  None,
    };

    let filepath = "/home/nuke/ctrlp.log";

    let log_file = File::create(filepath)?;

    WriteLogger::init(log_level_filter, config, log_file).unwrap();

    Ok(())
}


fn start_program() -> Result<(), Box<Error>> {
    info!("Connection to neovim");
    let (sender, receiver) = mpsc::channel();
    let mut session = Session::new_parent()?;
    session.start_event_loop_handler(NeovimHandler(sender));
    let nvim = Neovim::new(session);

    start_event_loop(receiver, nvim);

    Ok(())
}


fn start_event_loop(receiver: mpsc::Receiver<Event>, mut nvim: Neovim) {
    info!("Starting event loop");
    loop {
        info!("Waiting");
        let payload = receiver.recv();
        println!("Received Payload: {:?}", payload);
        match payload {
            Ok(Event::Shutdown) => {
                info!("Shutting down");
                break;
            }
            Ok(Event::Search) => {
                search(Event::Shutdown, &nvim);
            }
            _ => info!("unhandled event")
        }
    }
}
