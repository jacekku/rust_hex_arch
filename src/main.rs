use std::sync::Arc;

use repositories::pokemon::InMemoryRepository;

mod api;
mod domain;
mod repositories;
extern crate serde;
#[macro_use]
extern crate rouille;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());
    api::serve("localhost:8000", repo);
}
