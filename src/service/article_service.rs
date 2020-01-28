use crate::dao::article_dao::ArticleDao;
use crate::entity::article::Article;
use crate::util::snowflake;
use crate::entity::article::ArticleListParams;
use crate::errors::article_error::ArticleError;
use crate::entity::page::Page;
use rocket::request::Form;
use chrono::Utc;

pub trait ArticleService {
    fn create(article: Article) -> Result<Article, ArticleError>;
    fn get(id: i64) -> Result<Article, ArticleError>;
    fn list(params: Form<ArticleListParams>) -> Result<Page<Article>, ArticleError>;
    fn update(id: i64, article: Article) -> Result<Article, ArticleError>;
    fn delete(id: i64) -> Result<Article, ArticleError>;
}

pub struct ArticleServiceImpl {}

impl ArticleService for ArticleServiceImpl {
    fn create(mut article: Article) -> Result<Article, ArticleError> {
        article.id = snowflake::id();
        article.create_time = Option::from(Utc::now().timestamp());
        article.modify_time = article.create_time;
        article.creator = Option::from(12345_i64);
        article.modifier = article.creator;
        ArticleDao::insert(article)
    }

    fn get(id: i64) -> Result<Article, ArticleError> {
        ArticleDao::get(id)
    }

    fn list(params: Form<ArticleListParams>) -> Result<Page<Article>, ArticleError> {
        ArticleDao::list(params)
    }

    fn update(id: i64, mut article: Article) -> Result<Article, ArticleError> {
        article.id = Option::from(id);
        ArticleDao::update(article)
    }

    fn delete(id: i64) -> Result<Article, ArticleError> {
        ArticleDao::delete(id)
    }
}
