mod endpoint_functions;

use sqlx::postgres::PgPoolOptions;
use std::env;


#[tokio::main]
async fn main() {
    // getting an environment variable whereby if missing will return/log an error
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    // Creating a new pool instance and will return an error if it fails to connect to the database
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Database connection failed");
    // Migrating the database and if it fails will raise an error
    sqlx::migrate!().run(&pool).await.expect("Database migration failed");
    // Call the function to get the Router instance
    let app =endpoint_functions::api_routes(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server running on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}