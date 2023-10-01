#[macro_use]
mod common;

use axum::Router;
use common::build_app;
#[cfg(all(
    test,
    feature = "axum-core",
    feature = "sqlite-store",
    feature = "moka-store"
))]
use tower_sessions::{sqlx::SqlitePool, MokaStore, SessionManagerLayer, SqliteStore};

#[cfg(all(
    test,
    feature = "axum-core",
    feature = "sqlite-store",
    feature = "moka-store"
))]
async fn app(max_age: Option<Duration>) -> Router {
    use tower_sessions::CachingSessionStore;

    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let sqlite_store = SqliteStore::new(pool);
    sqlite_store.migrate().await.unwrap();
    let moka_store = MokaStore::new(None);
    let caching_store = CachingSessionStore::new(moka_store, sqlite_store);
    let session_manager = SessionManagerLayer::new(caching_store).with_secure(true);

    build_app(session_manager, max_age)
}

#[cfg(all(
    test,
    feature = "axum-core",
    feature = "sqlite-store",
    feature = "moka-store"
))]
route_tests!(app);