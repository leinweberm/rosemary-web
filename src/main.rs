use std::net::Ipv4Addr;
use sqlx::{Pool, Postgres};
use warp::Filter;

mod requests;
mod utils;
mod database;
mod client;
mod config;
mod errors;

extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	pretty_env_logger::init();

	let _init_config = config::load::init().await?;
	let _test_config = config::load::test().await?;
	debug!("app config loaded and tested");

	debug!(target: "app", "Database connecting");
	let client: Pool<Postgres> = database::connection::init_connection().await?;
	debug!(target: "app", "Database connected");

	utils::auth::token::set_keys().await?;
	debug!(target: "app", "JWT secrets initialized");

	let rows: (i64,) = sqlx::query_as("SELECT $1")
		.bind(150_i64)
		.fetch_one(&client)
		.await?;

	assert_eq!(rows.0, 150_i64);
	debug!(target: "app", "Database connection checked");

	let routes = requests::router::router().recover(errors::api_error::handle_rejection);
	debug!(target: "app", "Router routes initialized");

	debug!(target: "app", "App is listening on {}:3030", Ipv4Addr::LOCALHOST);
	warp::serve(routes)
		.run((Ipv4Addr::LOCALHOST, 3030))
		.await;

	Ok(())
}