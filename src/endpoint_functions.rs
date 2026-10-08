use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{get, post};
use axum::response::Html;
use axum_swagger_ui::{swagger_ui};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Pool, Postgres};


/// Defining app routes with what they will do(CRUD tasks) or show
pub fn api_routes(pool: Pool<Postgres>) -> Router {
    let doc_url = "/swagger/openapi.json";
    let app = Router::new().route("/", get(root))
        .route("/swagger", get(move|| async move { Html::from(swagger_ui(doc_url)) }))
        .route(doc_url, get(|| async { include_str!("openapi.json") }))
        .route("/users", post(create_user).get(list_users))
        .route("/users/{id}", get(get_user).delete(delete_user).put(update_user))
        .with_state(pool);

    return app;
}



/// User instance for whenever we are creating a user
#[derive(Deserialize)]
struct UserPayLoad{
    name: String,
    email: String,
}

/// User instance for whenever fetching a user
#[derive(Serialize, FromRow)]
struct User{
    id: i32,
    name: String,
    email: String,
}

/// Endpoint handler.
///
/// Test endpoint
async fn root() -> &'static str {
    "Welcome to User management api"
}
/// Fetch all users
///
/// Will return an internal server error in case of an error
 async fn list_users(State(pool):State<PgPool>)-> Result<Json<Vec<User>>, StatusCode>{
    sqlx::query_as::<_,User>("SELECT * FROM users").fetch_all(&pool).await
        .map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}


/// Creating user instance
 async fn create_user(
    State(pool):State<PgPool>,
    Json(payload):Json<UserPayLoad>,
)-> Result<( StatusCode,Json<User>), StatusCode>{
    sqlx::query_as::<_, User>
        ("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
        .bind(&payload.name)
        .bind(&payload.email)
        .fetch_one(&pool)
        .await.
        map(|user|(StatusCode::CREATED, Json(user)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_user(State(pool):State<PgPool>, Path(id):Path<i32>) -> Result<Json<User>, StatusCode>{
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(        id    )
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_|StatusCode::NOT_FOUND)
}


/// Updating user instance. Requires user payload and fetches using user.
/// Since id is readonly and cann
async fn update_user(State(pool):State<PgPool>, Path(id):Path<i32>, Json(payload):Json<UserPayLoad>)-> Result<Json<User>, StatusCode>{
    sqlx::query_as::<_, User>("UPDATE users SET name = $1, email = $2 WHERE id = $3")
        .bind(payload.name)
        .bind(payload.email)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
async fn delete_user(State(pool):State<PgPool>, Path(id):Path<i32>) -> Result<StatusCode, StatusCode>{
    let res = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_|StatusCode::INTERNAL_SERVER_ERROR);
    if res?.rows_affected() == 0{ 
        Err(StatusCode::NOT_FOUND) 
    }else {
        Ok(StatusCode::NO_CONTENT) 
    }
}