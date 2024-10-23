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
		debug!(target: "api", "paintings_get_all:client - database client aquired");

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
        let rows = sqlx::query_as::<_, Painting>(&query)
            .fetch_all(&*rows_client)
            .await
            .expect("Failed to select painting rows");
        rows
    });

    let (count, rows) = tokio::join!(count_task, rows_task);

    result.count = match count {
        Ok(count) => count,
        Err(error) => {
					error!(target: "api", "paintings_get_all:error - failed to count {}", error);
					0
				},
    };
		debug!(target: "api", "paintings_get_all:count - {}", &result.count);

    result.rows = match rows {
        Ok(rows) => rows,
        Err(error) => {
					error!(target: "api", "paintings_get_ll:error - failed to get rows {}", error);
					Vec::new()
				},
    };
		debug!(target: "api", "paintings_get_all:rows - {:?}", &result.rows);

    Ok(warp::reply::json(&result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("api"))
        .and(path("v1.0"))
        .and(path("paintings"))
        .and(query::<GetPaintingsQuery>())
        .and_then(get_paintings)
        .with(warp::log("api"))
}