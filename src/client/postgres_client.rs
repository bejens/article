use rustorm::Pool;

pub fn init_postgres() -> Pool {
    let mut pool = Pool::new();
    let db_url = "postgres://postgres:admin@localhost:5432/article";
    let _result = pool.ensure(db_url);
    pool
}
