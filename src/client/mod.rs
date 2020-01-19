use rustorm::EntityManager;
use std::sync::Once;

mod postgres_client;

static INIT: Once = Once::new();

pub fn pg_client() -> EntityManager {
    postgres_client::init_postgres()
}