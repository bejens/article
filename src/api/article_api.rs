use crate::entity::article::Article;
use crate::service::article_service;
use crate::service::article_service::ArticleService;
use rocket_contrib::json::Json;

#[post("/", format = "json", data = "<article>")]
pub fn create(article: Json<Article>) -> Json<Result<Article, String>> {
    let result = article_service::ArticleServiceImpl::create(article.0);
    return Json(result);
}

#[get("/<id>", format = "json")]
pub fn get(id: i64) -> Json<Result<Article, String>> {
    let result = article_service::ArticleServiceImpl::get(id);
    return Json(result);
}

#[put("/<id>", format = "json", data = "<article>")]
pub fn update(id: i64, article: Json<Article>) -> Json<Result<Article, String>> {
    let result = article_service::ArticleServiceImpl::update(id, article.0);
    return Json(result);
}

#[delete("/<id>")]
pub fn delete(id: i64) -> Json<Result<Article, String>> {
    let result = article_service::ArticleServiceImpl::delete(id);
    return Json(result);
}
