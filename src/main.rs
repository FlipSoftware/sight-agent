#![warn(clippy::all)]
use color_eyre::eyre::Result;
use colored::*;
use handle_errors::handle_errors;
use tracing_subscriber::fmt::format::FmtSpan;
use uuid::Uuid;
use warp::{http::Method, Filter};

use crate::routes::{
    kb_list::{add_kb, delete_kb, get_kb, get_kb_by_id, update_kb},
    reply::add_reply,
};

mod config;
mod db;
mod routes;
mod types;

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    let cfg = config::Config::new().expect("Config module failed");

    let log_rec = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle-errors={},rust-kb-center={},warp={}",
            cfg.log_level, cfg.log_level, cfg.log_level
        )
    });

    tracing_subscriber::fmt()
        .with_env_filter(log_rec)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db = db::Database::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        cfg.db_user, cfg.db_user_password, cfg.db_host, cfg.db_port, cfg.db_name
    ))
    .await
    .map_err(handle_errors::Error::DBQueryError)?;

    sqlx::migrate!()
        .run(&db.clone().connection)
        .await
        .map_err(handle_errors::Error::MigrationError)?;

    let db_access = warp::any().map(move || db.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let get_kb = warp::get()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and(warp::query())
        .and(db_access.clone())
        .and_then(get_kb)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "GET kb request",
                method = %info.method(),
                path = %info.path(),
                id = %Uuid::new_v4(),
            )
        }));

    let get_kb_by_id = warp::get()
        .and(warp::path("kb"))
        .and(warp::path::param())
        .and(db_access.clone())
        .and_then(get_kb_by_id)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "GET kb/{id} request",
                method = %info.method(),
                path = %info.path(),
                id = %Uuid::new_v4(),
            )
        }));

    let add_kb = warp::post()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and(routes::auth::auth())
        .and(db_access.clone())
        .and(warp::body::json())
        .and_then(add_kb);

    let update_kb = warp::put()
        .and(warp::path("kb"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::auth::auth())
        .and(db_access.clone())
        .and(warp::body::json())
        .and_then(update_kb);

    let delete_kb = warp::post()
        .and(warp::path("kb"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::auth::auth())
        .and(db_access.clone())
        .and_then(delete_kb);

    let add_reply = warp::post()
        .and(warp::path("kb"))
        .and(warp::path::end())
        .and(routes::auth::auth())
        .and(db_access.clone())
        .and(warp::body::form())
        .and_then(add_reply);

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::path::end())
        .and(db_access.clone())
        .and(warp::body::json())
        .and_then(routes::auth::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(db_access.clone())
        .and(warp::body::json())
        .and_then(routes::auth::login);

    let router = get_kb
        .or(register)
        .or(login)
        .or(add_kb)
        .or(get_kb_by_id)
        .or(update_kb)
        .or(delete_kb)
        .or(add_reply)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors);
    println!(
        "{}: {} ",
        "Running server on port".green().bold(),
        cfg.port.to_string().on_bright_yellow().black().blink()
    );

    tracing::info!("KB build ID {}", env!("RUST_KB_CENTER_VERSION"));

    warp::serve(router).run(([127, 0, 0, 1], cfg.port)).await;

    Ok(())
}
