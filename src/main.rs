//! Run with
//!
//! ```not_rust
//! cargo run -p example-diesel-postgres
//! ```
//!
//! Checkout the [diesel webpage](https://diesel.rs) for
//! longer guides about diesel
//!
//! Checkout the [crates.io source code](https://github.com/rust-lang/crates.io/)
//! for a real world application using axum and diesel

pub mod models;
pub mod schema;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use models::{NewUser, SourceFile};
use std::net::SocketAddr;

// this embeddes the migrations into the application binary
// the migration path is releative to the `CARGO_MANIFEST_DIR`
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::sqlite::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::sqlite::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // build our application with some routes
    let app = Router::new()
        .route("/user/list", get(list_users))
        .route("/user/create", post(create_user))
        .fallback(fallback_handler)
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fallback_handler(uri: axum::http::Uri) -> (StatusCode, String) {
    println!("Path not found {}", uri.path());
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

async fn create_user(
    State(pool): State<deadpool_diesel::sqlite::Pool>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<SourceFile>, (StatusCode, String)> {
    use self::schema::source_files::dsl::*;
    use diesel::RunQueryDsl;
    use diesel::SelectableHelper;

    println!("create_user: {:?}", new_user);

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(source_files)
                .values(new_user)
                .returning(SourceFile::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn list_users(
    State(pool): State<deadpool_diesel::sqlite::Pool>,
) -> Result<Json<Vec<SourceFile>>, (StatusCode, String)> {
    use self::schema::source_files::dsl::*;
    use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| source_files.select(SourceFile::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
