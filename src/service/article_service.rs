use crate::dao::article_dao::ArticleDao;
use crate::entity::article::Article;
use crate::entity::article::ArticleListParams;
use crate::entity::page::Page;
use crate::errors::article_error::ArticleError;
use crate::util::snowflake;
use chrono::Utc;

pub struct ArticleService {}

impl ArticleService {
    pub async fn create(mut article: Article) -> Result<Article, ArticleError> {
        article.id = snowflake::id();
        article.create_time = Option::from(Utc::now().timestamp());
        article.modify_time = article.create_time;
        article.creator = Option::from(12345_i64);
        article.modifier = article.creator;
        ArticleDao::insert(article)
    }

    pub async fn get(id: i64) -> Result<Article, ArticleError> {
        ArticleDao::get(id)
    }

    pub async fn list(params: ArticleListParams) -> Result<Page<Article>, ArticleError> {
        ArticleDao::list(params)
    }

    pub async fn update(id: i64, mut article: Article) -> Result<Article, ArticleError> {
        article.id = Option::from(id);
        ArticleDao::update(article)
    }

    pub async fn delete(id: i64) -> Result<Article, ArticleError> {
        ArticleDao::delete(id)
    }
}
