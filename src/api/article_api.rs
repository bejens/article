use crate::entity::article::Article;
use crate::service::article_service;
use crate::service::article_service::ArticleService;
use rocket_contrib::json::Json;

#[post("/", format = "json", data = "<article>")]
pub fn create(article: Json<Article>) -> Json<Result<Article, String>> {
    let result = article_service::ArticleServiceImpl::create(article.0);
    return Json(result);
}
