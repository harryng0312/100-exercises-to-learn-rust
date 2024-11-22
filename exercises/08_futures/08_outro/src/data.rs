// #[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
// pub struct TicketId(u64);

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "ID")]
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: String,
    pub description: String,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    #[default]
    #[serde(rename = "TODO")]
    ToDo,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "DONE")]
    Done,
}