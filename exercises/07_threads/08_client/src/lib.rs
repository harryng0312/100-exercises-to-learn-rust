use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    s_sender: Sender<Command>,
    // get_sender: Sender<Option<Ticket>>,
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (c_sender, c_receiver) = channel();
        self.s_sender.send(Command::Insert {draft, response_channel: c_sender}).expect("failed to send draft");
        loop {
            match c_receiver.recv() {
                Ok(command) => {
                    return command;
                }
                _ => {}
            }
        };
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (c_sender, c_receiver) = channel();
        self.s_sender.send(Command::Get {id, response_channel: c_sender}).expect("failed to send draft");
        loop {
            match c_receiver.recv() {
                Ok(command) => {
                    return command;
                }
                _ => {}
            }
        };
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    // let (c_sender, c_receiver) = std::sync::mpsc::channel();
    TicketStoreClient {
        s_sender: sender
    }
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
