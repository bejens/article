use rustorm::{rustorm_dao, FromDao};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page<T> {
    pub total: i64,
    pub size: i64,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, FromDao)]
pub struct Count {
    pub count: i64,
}
