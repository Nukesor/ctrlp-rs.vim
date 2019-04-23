use std::sync::mpsc;
use log::{info, error};
use neovim_lib::{Handler, RequestHandler, Value};

use crate::event::Event;

pub struct NeovimHandler(pub mpsc::Sender<Event>);

impl NeovimHandler {
}


impl RequestHandler for NeovimHandler {
    /// We don't handle requests yet.
    fn handle_request(&mut self, _name: &str, _args: Vec<Value>) -> Result<Value, Value> {
        Err(Value::from("Not supported"))
    }
}


impl Handler for NeovimHandler {
    fn handle_notify(&mut self, name: &str, args: Vec<Value>) {
        info!("Incoming event: {}", name);
        match name {
            "shutdown" => {
                if let Err(reason) = self.0.send(Event::Shutdown) {
                    error!("{}", reason);
                }
            }
            "search" => {
                if let Err(reason) = self.0.send(Event::Search) {
                    error!("{}", reason);
                }
            }
            _ => {
                error!("Unknown event {}", name);
            }
        }
    }
}