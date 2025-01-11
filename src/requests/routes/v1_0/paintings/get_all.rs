use std::sync::Arc;
use warp::{Filter, Rejection, Reply, path, query};
use sqlx::{Pool, Postgres};

use crate::requests::dto::get_paintings_query::GetPaintingsQuery;
use crate::database::models::painting::Painting;
use crate::database::connection::get_client;
use crate::requests::dto::paginated_result::PaginatedResult;

async fn get_paintings(query: GetPaintingsQuery) -> Result<impl Reply, Rejection> {
	let mut result: PaginatedResult<Painting> = PaginatedResult::new();
	let client: Arc<Pool<Postgres>> = Arc::new(get_client().await.unwrap().clone());
	debug!(target: "api", "paintings:get_all - database client aquired");

	let count_client = Arc::clone(&client);
	let count_task = tokio::spawn(async move {
		let (count,): (i64,) = sqlx::query_as(&Painting::count_all_query())
			.fetch_one(&*count_client)
			.await
			.expect("Failed to count painting rows");

			count
	});

	let rows_client = Arc::clone(&client);
	let rows_task = tokio::spawn(async move {
		let limit = query.limit.unwrap_or(20);
		let offset = query.offset.unwrap_or(0);
		let query = Painting::get_all_query(limit, offset);
		debug!(target: "db", "paintings:get_all - Painting::get_all_query {}", &query);
		let rows = sqlx::query_as::<_, Painting>(&query)
			.fetch_all(&*rows_client)
			.await
			.expect("Failed to select painting rows");
		rows
	});

	let (count, rows) = tokio::join!(count_task, rows_task);

	result.count = count.unwrap_or_else(|error| {
		error!(target: "api", "paintings:get_all - failed to count {}", error);
		0
	});
	debug!(target: "api", "paintings:get_all - {}", &result.count);

	result.rows = rows.unwrap_or_else(|error| {
		error!(target: "api", "paintings:get_all - failed to get rows {}", error);
		Vec::new()
	});
	debug!(target: "api", "paintings:get_all - {:?}", &result.rows);

	Ok(warp::reply::json(&result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
	warp::get()
		.and(path("api"))
		.and(path("v1.0"))
		.and(path("paintings"))
		.and(path::end())
		.and(query::<GetPaintingsQuery>())
		.and_then(get_paintings)
}