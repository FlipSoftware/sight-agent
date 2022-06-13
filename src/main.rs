#![warn(clippy::all)]
use handle_errors::handle_errors;
use warp::{http::Method, Filter};

use crate::{
    db::Database,
    routes::{
        info::add_info_to_kb,
        knowledge_base::{add_kb, delete_kb, get_kb, update_kb},
    },
};

mod db;
mod routes;
mod types;

#[tokio::main]
async fn main() {
    let db = Database::new();
    let db_access = warp::any().map(move || db.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let kb_data = warp::get()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and(warp::query())
        .and(db_access.clone())
        .and_then(get_kb);

    let add_kb = warp::post()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and(db_access.clone())
        .and(warp::body::json())
        .and_then(add_kb);

    let update_kb = warp::put()
        .and(warp::path("kb"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(db_access.clone())
        .and(warp::body::json())
        .and_then(update_kb);

    let delete_kb = warp::put()
        .and(warp::path("kb"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(db_access.clone())
        .and_then(delete_kb);

    let add_info_to_kb = warp::post()
        .and(warp::path("info"))
        .and(warp::path::end())
        .and(db_access.clone())
        .and(warp::body::form())
        .and_then(add_info_to_kb);

    // TODO: add :id route

    let router = kb_data
        .or(add_kb)
        .or(update_kb)
        .or(delete_kb)
        .or(add_info_to_kb)
        .with(cors)
        .recover(handle_errors);
    println!("Running server on port: 8080");
    warp::serve(router).run(([127, 0, 0, 1], 8080)).await;
}

// SECTION: Types

// SECTION: Functions

// SECTION: Mods
