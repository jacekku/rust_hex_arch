use std::sync::Arc;

use crate::{api::Status, domain::create_pokemon, repositories::pokemon::Repository};
use rouille;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[derive(Serialize)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn serve(req: &rouille::Request, repo: Arc<dyn Repository>) -> rouille::Response {
    let req = match rouille::input::json_input::<Request>(req) {
        Ok(req) => create_pokemon::Request {
            number: req.number,
            name: req.name,
            types: req.types,
        },
        _ => return rouille::Response::from(Status::BadRequest),
    };

    match create_pokemon::execute(repo, req) {
        Ok(create_pokemon::Response {
            number,
            name,
            types,
        }) => rouille::Response::json(&Response {
            number,
            name,
            types,
        }),
        Err(create_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(create_pokemon::Error::Conflict) => rouille::Response::from(Status::Conflict),
        Err(create_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
    }
}
