// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, RecvError, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, RecvError> {
        let (sender, receiver) = sync_channel(2);
        self.sender.send(Command::Insert { draft, response_channel: sender }).expect("can't send draft draft");
        loop {
            return receiver.recv();
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, RecvError> {
        let (sender, receiver) = sync_channel(2);
        self.sender.send(Command::Get { id, response_channel: sender }).expect("can't send draft draft");
        loop {
            return receiver.recv();
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                   draft,
                   response_channel,
               }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).expect("failed to send ticket");
            }
            Ok(Command::Get {
                   id,
                   response_channel,
               }) => {
                let ticket = store.get(id);
                if let Some(ticket) = ticket {
                    response_channel.send(Some(ticket.clone())).expect("failed to send ticket");
                } else {
                    response_channel.send(None).expect("failed to send ticket");
                }
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
