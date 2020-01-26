use crate::dao::article_dao::ArticleDao;
use crate::entity::article::Article;
use crate::util::snowflake;
use chrono::Utc;

pub trait ArticleService {
    fn create(article: Article) -> Result<Article, String>;
    fn get(id: i64) -> Result<Article, String>;
    fn update(id: i64, article: Article) -> Result<Article, String>;
    fn delete(id: i64) -> Result<Article, String>;
}

pub struct ArticleServiceImpl {}

impl ArticleService for ArticleServiceImpl {
    fn create(mut article: Article) -> Result<Article, String> {
        article.id = snowflake::id();
        article.create_time = Option::from(Utc::now().timestamp());
        article.modify_time = article.create_time;
        article.creator = Option::from(12345_i64);
        article.modifier = article.creator;
        ArticleDao::insert(article)
    }

    fn get(id: i64) -> Result<Article, String> {
        ArticleDao::get(id)
    }

    fn update(id: i64, mut article: Article) -> Result<Article, String> {
        article.id = Option::from(id);
        ArticleDao::update(article)
    }

    fn delete(id: i64) -> Result<Article, String> {
        ArticleDao::delete(id)
    }
}
