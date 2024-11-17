use crate::data::TicketDraft;
use crate::store::TicketStore;
use lazy_static::lazy_static;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
lazy_static! {
    static ref TICKETSTORE: Arc<RwLock<TicketStore>> = Arc::new(RwLock::new(TicketStore::new()));
}

pub fn server(receiver: Receiver<Command>) {
    loop {
        if let Ok(command) = receiver.recv() {
            match command {
                Command::Insert(_draft) => {
                    let mut ticketstore = TICKETSTORE.write().expect("Cannot write to TicketStore");
                    ticketstore.add_ticket(_draft);
                }
            }
        }
    }
}
