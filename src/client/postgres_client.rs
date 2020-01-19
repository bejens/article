use rustorm::{EntityManager, Pool};

pub fn init_postgres() -> EntityManager {
    let mut client = Pool::new();
    let db_url = "postgres://postgres:p0stgr3s@localhost/sakila";
    client.em(db_url).unwrap()
}