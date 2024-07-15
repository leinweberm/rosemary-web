use postgres::types::ToSql;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply, path, query};

use crate::database::models::generics::Translation;
use crate::requests::dto::get_paintings_query::GetPaintingsQuery;
use crate::requests::dto::paginated_result::PaginatedResult;
use crate::database::connection::get_client;
use crate::database::models::painting::{Painting, PaintingImage};

async fn get_paintings(query: GetPaintingsQuery) -> Result<impl Reply, Rejection> {
    let mut result: PaginatedResult<Painting> = PaginatedResult::new();

    let client: &tokio_postgres::Client = get_client().await.unwrap();

    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);
    let order = query.order.unwrap_or("DESC".to_string());
    let sort = query.sort.unwrap_or("created".to_string());
    let search = query.search.unwrap_or("".to_string());

    let search_clone = search.clone();

    let count_query = tokio::spawn(async move {
        let mut query_string = String::from("SELECT COUNT(*) FROM paintings");
        let mut params = vec![];
        let search_param;

        if &search_clone != "" {
            query_string.push_str(" WHERE painting_title ILIKE $1
            OR painting_description ILIKE $1 OR ");
            search_param = format!("%{}%", &search_clone);
            params.push(&search_param as &(dyn ToSql + Sync));
        }

        let rows = client
            .query(&query_string, &params)
            .await
            .unwrap();

        let count: i64 = rows[0].get(0);
        count
    });

    let rows_query = tokio::spawn(async move {
        let mut params = vec![
            &limit as &(dyn ToSql + Sync),
            &offset as &(dyn ToSql + Sync),
            &sort as &(dyn ToSql + Sync),
            &order as &(dyn ToSql + Sync),
        ];

        let mut search_query =String::from("");

        if search != "" {
            params.push(&format!("%{}%", search) as &(dyn ToSql + Sync));
            search_query = String::from("AND (p.painting_title ILIKE $5 OR p.painting_description ILIKE $5)");
        }

        let mut query_string = String::from(format!(r#"
            SELECT
                p.*,
                pi.id AS preview_id,
                pi.preview AS preview_preview,
                pi.url AS preview_url,
                pi.alt AS preview_alt,
                pi.title AS preview_title,
                pi.painting_id AS preview_painting_id
            FROM paintings p
            LEFT JOIN painting_images AS pi ON pi.painting_id = p.id AND pi.preview = true
            WHERE p.deleted IS NULL
            {}
            LIMIT $1 OFFSET $2
            ORDER BY $3 $4

        "#, &search_query));

        let rows = client
            .query(&query_string, &params)
            .await
            .unwrap();

        let result = rows
            .iter()
            .map(|row| {
                let painting_title_value: serde_json::Value = row.get("painting_title");
                let painting_title: Option<Translation> = serde_json::from_value(painting_title_value).unwrap();

                Painting {
                    id: row.get("id") as Uuid,
                    created: row.get("created"),
                    deleted: row.get("deleted"),
                    price: row.get("price"),
                    painting_title,
                    painting_description: row.get("painting_description"),
                    data: row.get("data"),
                    width: row.get("width"),
                    height: row.get("height"),
                    preview: PaintingImage {
                        id: row.get("preview_id") as Uuid,
                        preview: row.get("preview_preview"),
                        url: row.get("preview_url"),
                        alt: row.get("preview_alt"),
                        title: row.get("preview_title"),
                        painting_id: row.get("preview_painting_id") as Uuid,
                    },
                }
            })
            .collect::<Vec<Painting>>();

        println!("paintings: {:?}", &result);

        result
    });

    let (count, rows) = tokio::try_join!(count_query, rows_query).unwrap();

    if !rows.is_empty() {
        result.rows = rows;
    }

    result.count = count;

    Ok(warp::reply::json(&result))
}

pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(path("/api/v1.0/paintings"))
        .and(query::<GetPaintingsQuery>())
        .and_then(get_paintings)
}