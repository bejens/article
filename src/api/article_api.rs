use crate::entity::article::Article;
use crate::entity::article::ArticleListParams;
use crate::entity::page::Page;
use crate::errors::article_error::ArticleError;
use crate::service::article_service;
use crate::service::article_service::ArticleService;
use rocket::request::Form;
use rocket_contrib::json::Json;

#[post("/", format = "json", data = "<article>")]
pub fn create(article: Json<Article>) -> Json<Result<Article, ArticleError>> {
    let result = article_service::ArticleServiceImpl::create(article.0);
    return Json(result);
}

#[get("/?<params..>")]
pub fn list(params: Form<ArticleListParams>) -> Json<Result<Page<Article>, ArticleError>> {
    let result = article_service::ArticleServiceImpl::list(params);
    return Json(result);
}

#[get("/<id>")]
pub fn get(id: i64) -> Json<Result<Article, ArticleError>> {
    let result = article_service::ArticleServiceImpl::get(id);
    return Json(result);
}

#[put("/<id>", format = "json", data = "<article>")]
pub fn update(id: i64, article: Json<Article>) -> Json<Result<Article, ArticleError>> {
    let result = article_service::ArticleServiceImpl::update(id, article.0);
    return Json(result);
}

#[delete("/<id>")]
pub fn delete(id: i64) -> Json<Result<Article, ArticleError>> {
    let result = article_service::ArticleServiceImpl::delete(id);
    return Json(result);
}
