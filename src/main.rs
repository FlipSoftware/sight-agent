#![warn(clippy::all)]
use clap::Parser;
use colored::*;
use config::Config;
use handle_errors::handle_errors;
use tracing_subscriber::fmt::format::FmtSpan;
use uuid::Uuid;
use warp::{http::Method, Filter};

use crate::{
    db::Database,
    routes::{
        kb_list::{add_kb, delete_kb, get_kb, get_kb_by_id, update_kb},
        reply::add_reply,
    },
};

mod db;
mod routes;
mod types;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let log_rec = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust-kb-center={},wawrp={}",
            args.log_level, args.log_level, args.log_level
        )
    });

    tracing_subscriber::fmt()
        .with_env_filter(log_rec)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db = Database::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        args.db_user, args.db_user_password, args.db_url, args.db_port, args.db_name
    ))
    .await;
    sqlx::migrate!()
        .run(&db.clone().connection)
        .await
        .expect("Can't complete connection: migrations failed");
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
        "{}: {}",
        "Running server on port".green().bold(),
        " 8080 ".on_bright_yellow().black().blink()
    );
    warp::serve(router).run(([127, 0, 0, 1], 8080)).await;
}

#[derive(clap::Parser, Debug, Default, serde::Deserialize, PartialEq)]
struct Args {
    /// Choose the level of logs printed on stdout terminal screen
    #[clap(short, long, default_value = "warn")]
    log_level: String,
    /// Database location. Local or remote
    #[clap(long, default_value = "localhost")]
    db_url: String,
    /// Exposed port to connect on the Database
    #[clap(long, default_value = "5432")]
    db_port: u16,
    /// User attached to the Database
    #[clap(long)]
    db_user: String,
    /// User password if needed to access the Database
    #[clap(long)]
    db_user_password: String,
    /// The Database name
    #[clap(long)]
    db_name: String,
    /// The exposed port over the WebSocket or localhost
    #[clap(long, default_value = "8080")]
    port: u16,
}
