use rustorm::{rustorm_dao, FromDao, ToColumnNames, ToDao, ToTableName};

#[derive(Serialize, Deserialize, Clone, Debug, FromDao, ToColumnNames, ToTableName, ToDao)]
pub struct Article {
    pub id: Option<i64>,
    pub title: String,
    pub keyword: String,
    pub article_type: i32,
    pub describe: String,
    pub cover: String,
    pub content: String,
    pub create_time: Option<i64>,
    pub creator: Option<i64>,
    pub modify_time: Option<i64>,
    pub modifier: Option<i64>,
}
