use crate::client;
use crate::entity::article::Article;
use rustorm::DbError;

pub struct ArticleDao {}

impl ArticleDao {
    pub fn insert(article: Article) -> Result<Article, String> {
        let mut em = client::connect();
        let article_result: Result<(), DbError> = em.single_insert(&article);
        match article_result {
            Ok(_data) => Ok(article),
            Err(err) => {
                println!("{:?}", err);
                Err("db error".to_string())
            }
        }
    }
}
