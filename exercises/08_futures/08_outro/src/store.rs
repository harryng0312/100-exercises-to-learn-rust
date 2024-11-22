use crate::data::{Status, Ticket, TicketDraft};
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub static TICKET_STORE: Lazy<Arc<RwLock<TicketStore>>> =
    Lazy::new(|| Arc::new(RwLock::new(TicketStore::new())));

pub struct TicketStore {
    tickets: BTreeMap<u64, Ticket>,
    count: u64,
}

impl TicketStore {
    pub(crate) fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            count: 0,
        }
    }

    pub fn add(&mut self, ticket: TicketDraft) -> Result<Ticket, Err(String)> {
        self.count += 1;
        let new_tk = Ticket{
            id: self.count,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo
        };
        self.tickets.insert(self.count, new_tk.clone());
        Ok(new_tk.clone())
    }

    pub fn get(&self, count: u64) -> Option<&'_ Ticket> {
        let ticket = self.tickets.get(&count);
        ticket
    }
}
