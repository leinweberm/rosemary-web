use warp::{Filter, Rejection, Reply, path, query};

use crate::requests::dto::get_paintings_query::GetPaintingsQuery;
use crate::database::models::painting::Painting;
use crate::database::connection::get_client;
use crate::requests::dto::paginated_result::PaginatedResult;

async fn get_paintings(query: GetPaintingsQuery) {
    let mut result: PaginatedResult<Painting> = PaginatedResult::new();
    let client: &tokio_postgres::Client = get_client().await.unwrap();


    let count_task = tokio::spawn(async move {
        let count = sqlx::query(r#"
            SELECT COUNT(*)
            FROM paintings
            WHERE deleted IS NULL
        "#)
            .fetch_one(&client)
            .await?;
        count
    });


    let rows_task = tokio::spawn(async move {
        let rows = sqlx::query_as::<_, Painting>(r#"
            SELECT
                p.id,
                p.created,
                p.deleted,
                p.price,
                p.painting_title,
                p.painting_description,
                p.data,
                p.width,
                p.height,
                p.preview
            FROM paintings p
            WHERE deleted IS NULL
            ORDER BY $1 $2
            LIMIT $3 OFFSET $4
        "#)
            .bind(query.sort.unwrap_or("created".to_string()))
            .bind(query.order.unwrap_or("DESC".to_string()))
            .bind(query.limit.unwrap_or(20) as f32)
            .bind(query.offset.unwrap_or(0) as f32)
            .fetch_all(&client);
            // .await?;
        rows
    });

    let (count, rows) = tokio::try_join!(count_task, rows_task).unwrap();
    println!("Count: {}", &count);
    println!("Rows: {}", &rows);

    Ok(warp::reply::json(&result));
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("/api/v1.0/paintings"))
        .and(query::<GetPaintingsQuery>())
        .and_then(get_paintings)
}
