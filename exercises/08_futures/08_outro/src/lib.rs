// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use crate::data::TicketDraft;
use crate::store::TICKET_STORE;
use actix_web::error::ErrorInternalServerError;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use std::ops::Deref;
use tokio::io::AsyncReadExt;

pub mod data;
pub mod store;
mod util;

fn get_ticket(id: web::Path<u64>) -> Result<impl Responder, Error> {
    if let Some(ref ticketstore_lck) = TICKET_STORE.read().ok() {
        let ticket = ticketstore_lck.get(id.into());
        Ok(HttpResponse::Ok().json(&ticket))
    } else {
        Err(ErrorInternalServerError("can not read ticket"))
    }
}

fn add_ticket(ticket: web::Json<TicketDraft>) -> Result<impl Responder, Error> {
    if let Ok(mut ticketstore) = TICKET_STORE.write() {
        match ticketstore.add(ticket.into_inner()) {
            Ok(ticket) => Ok(HttpResponse::Ok().json(&ticket)),
            Err(msg) => Err(ErrorInternalServerError(msg)),
        }
    } else {
        Err(ErrorInternalServerError("can not read ticket"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(Files::new("/static", "./static"))
            .route("/ticket/{id}", web::get().to(get_ticket))
            .route("/ticket/add", web::post().to(add_ticket))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// fn main() {
//     let sys = tokio::runtime::Runtime::new().unwrap();
//     sys.block_on(async {
//         HttpServer::new(|| {
//             App::new()
//                 .route("/", web::get().to(greet))
//         })
//             .bind("127.0.0.1:8080")
//             .unwrap()
//             .run()
//             .await
//             .unwrap();
//     });
// }
