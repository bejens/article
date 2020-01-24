use rustorm::{EntityManager, Pool};
use std::sync::{Arc, Mutex};

mod postgres_client;

pub fn connect() -> EntityManager {
    static mut SINGLETONS: Option<Arc<Mutex<Pool>>> = None;
    let db_url = "postgres://postgres:admin@localhost:5432/article";

    unsafe {
        SINGLETONS
            .get_or_insert_with(|| Arc::new(Mutex::new(postgres_client::init_postgres())))
            .clone()
            .try_lock()
            .unwrap()
            .em(db_url)
            .unwrap()
    }
}
