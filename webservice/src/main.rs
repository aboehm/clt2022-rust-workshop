#[macro_use]
extern crate rocket;

use async_trait::async_trait;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::Executor;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct Item {
    uuid: Uuid,
    text: Option<String>,
    created: DateTime<Utc>,
    #[allow(dead_code)]
    updated: Option<DateTime<Utc>>,
    version: u32,
}

#[async_trait]
trait ItemDatabase: Send + Sync {
    async fn create(&self) -> Uuid;
    async fn get(&self, uuid: Uuid) -> Option<Item>;
    async fn list(&self) -> Vec<Item>;
}

struct AppDatabase {
    pool: SqlitePool,
}

#[async_trait]
impl ItemDatabase for AppDatabase {
    async fn create(&self) -> Uuid {
        let item_id = Uuid::new_v4();
        let now = Utc::now();

        let mut conn = self.pool.acquire().await.unwrap();
        sqlx::query(
            r#"
                INSERT INTO items (uuid, created, version)
                VALUES ($1, $2, $3)
            "#,
        )
        .bind(&item_id)
        .bind(now)
        .bind(1 as u32)
        .execute(&mut conn)
        .await
        .unwrap();

        item_id
    }

    async fn get(&self, uuid: Uuid) -> Option<Item> {
        sqlx::query_as(
            r#"
                SELECT uuid, text, created, updated, version
                FROM items
                WHERE uuid = $1
            "#,
        )
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await
        .unwrap()
    }

    async fn list(&self) -> Vec<Item> {
        sqlx::query_as::<_, Item>(
            r#"
                SELECT uuid, text, created, updated, version FROM items
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}

#[derive(Debug)]
enum ApplicationPart {
    Database,
    Webserver,
}

struct ApplicationError {
    part: ApplicationPart,
    cause: String,
}

impl From<sqlx::Error> for ApplicationError {
    fn from(error: sqlx::Error) -> ApplicationError {
        ApplicationError {
            part: ApplicationPart::Database,
            cause: format!("{:?}", error),
        }
    }
}

impl From<rocket::Error> for ApplicationError {
    fn from(error: rocket::Error) -> ApplicationError {
        ApplicationError {
            part: ApplicationPart::Webserver,
            cause: format!("{:?}", error),
        }
    }
}

impl std::fmt::Debug for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}: {}", self.part, self.cause)
    }
}

impl AppDatabase {
    pub async fn connect() -> Result<Self, ApplicationError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(4)
            .connect("sqlite://:memory:")
            .await?;

        pool.execute(
            r#"
            CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uuid VARCHAR(64) NOT NULL,
                text VARCHAR(256),
                created DATETIME NOT NULL,
                updated DATETIME,
                version INTEGER NOT NULL,
                UNIQUE(uuid)
            )
        "#,
        )
        .await?;

        Ok(AppDatabase { pool })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hallo CLT 2022"
}

#[derive(Debug, Deserialize, Serialize)]
struct ListItemsResponse {
    items: Vec<Uuid>,
}

#[get("/")]
async fn list_items(database: &State<Box<dyn ItemDatabase + 'static>>) -> Json<ListItemsResponse> {
    let items = database.list().await;
    Json(ListItemsResponse {
        items: items.iter().map(|i| i.uuid).collect::<Vec<Uuid>>(),
    })
}

#[derive(Debug, Deserialize, Serialize)]
struct NewItemResponse {
    item_id: Uuid,
}

#[post("/")]
async fn new_item(database: &State<Box<dyn ItemDatabase + 'static>>) -> Json<NewItemResponse> {
    let item_id = database.create().await;
    Json(NewItemResponse { item_id })
}

#[derive(Debug, Deserialize, Serialize)]
struct GetItemResponse {
    text: Option<String>,
    created: String,
    updated: Option<String>,
    version: u32,
}

#[get("/<item_id>")]
async fn get_item(
    database: &State<Box<dyn ItemDatabase + 'static>>,
    item_id: &str,
) -> Result<Json<GetItemResponse>, NotFound<Json<ErrorResponse>>> {
    let uuid_id = Uuid::parse_str(item_id).unwrap();
    match database.get(uuid_id).await {
        Some(item) => {
            let created: DateTime<Utc> = item.created.into();
            Ok(Json(GetItemResponse {
                text: item.text,
                created: created.to_rfc3339(),
                updated: None,
                version: item.version,
            }))
        },
        None => Err(NotFound(Json(ErrorResponse {
            message: "Item not found".to_string(),
        }))),
    }
}

#[delete("/<item_id>")]
async fn delete_item(item_id: &str) -> String {
    format!("removed {}", item_id)
}

fn build_server(db: Box<dyn ItemDatabase + 'static>) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount(
            "/items",
            routes![new_item, get_item, delete_item, list_items],
        )
}

#[rocket::main]
async fn main() -> Result<(), ApplicationError> {
    let db: Box<dyn ItemDatabase> = Box::new(AppDatabase::connect().await?);

    build_server(db)
        .launch()
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::asynchronous::Client;
    use sqlx::types::chrono::{DateTime, Utc};

    async fn get_db_async() -> Box<dyn ItemDatabase + 'static> {
        Box::new(AppDatabase::connect().await.expect("A database"))
    }

    #[rocket::async_test]
    #[serial_test::serial]
    async fn empty_list_of_items() {
        let rocket = build_server(get_db_async().await);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.get("/items").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let data: ListItemsResponse =
            rocket::serde::json::from_str(&response.into_string().await.unwrap())
                .expect("Parsed json");
        assert!(data.items.is_empty());
    }

    #[rocket::async_test]
    #[serial_test::serial]
    async fn add_and_get_item() {
        let now = Utc::now();

        let rocket = build_server(get_db_async().await);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.post("/items").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let new_item: NewItemResponse =
            rocket::serde::json::from_str(&response.into_string().await.unwrap())
            .expect("Parsed json");

        let response = client.get(format!("/items/{}", new_item.item_id))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let exist_item: GetItemResponse =
            rocket::serde::json::from_str(&response.into_string().await.unwrap())
            .expect("Parsed json");

        let created = DateTime::parse_from_rfc3339(&exist_item.created)
            .expect("A valid timestamp");

        assert!(now < created);
    }
}
