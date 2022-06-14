#![warn(clippy::all)]
use colored::*;
use handle_errors::handle_errors;
use uuid::Uuid;
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
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    log::error!("{} Capturing stderr", " ERROR ".on_red().white().bold());
    log::info!("{} Capturing stdout", " INFO ".on_cyan().black().bold());
    log::warn!(
        "{} Capture stdout warnings",
        " WARN ".on_yellow().black().bold()
    );

    let log = warp::log::custom(|info| {
        let state_log = " STATE LOG ".on_truecolor(0, 128, 0);
        let capture = " rec ".red().blink();
        log::info!(
            "{state_log}{capture}\n {} {} {} {:?} from {} with {:#?}",
            match info.method() {
                get @ &Method::GET => get.to_string().green().bold(),
                post @ &Method::POST => post.to_string().yellow().bold(),
                put @ &Method::PUT => put.to_string().purple().bold(),
                delete @ &Method::DELETE => delete.to_string().red().bold(),
                _ => "".to_string().on_black(),
            },
            info.path(),
            info.status().to_string().blue().bold(),
            info.elapsed(),
            info.remote_addr().unwrap().to_string().yellow(),
            info.request_headers()
        );
    });

    let db = Database::new();
    let db_access = warp::any().map(move || db.clone());

    let id_access = warp::any().map(|| Uuid::new_v4().to_string());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let get_kb = warp::get()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and(warp::query())
        .and(db_access.clone())
        .and(id_access)
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

    let router = get_kb
        .or(add_kb)
        .or(update_kb)
        .or(delete_kb)
        .or(add_info_to_kb)
        .with(cors)
        .with(log)
        .recover(handle_errors);
    println!("Running server on port: 8080");
    warp::serve(router).run(([127, 0, 0, 1], 8080)).await;
}

// SECTION: Types

// SECTION: Functions

// SECTION: Mods
