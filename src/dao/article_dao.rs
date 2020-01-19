use crate::entity::article::Article;
use crate::client::pg_client;
use std::result::Result;

pub struct ArticleDao {}

impl ArticleDao {
    pub fn insert(article: Article) -> Result<Article, String> {
        let mut client = pg_client();
        if match client.single_insert(&article) {
            Ok(row) =>  _
            Err(err) => err;
        }
        return Ok(article);
    }
}

