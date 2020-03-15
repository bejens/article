use crate::entity::article::Article;
use crate::entity::article::ArticleListParams;
use crate::service::article_service::ArticleService;
use actix_web::{get, post, delete, put, web, Responder, HttpResponse};

#[post("")]
pub async fn create(article: web::Json<Article>) -> impl Responder {
    let result = ArticleService::create(article.0).await;
    return HttpResponse::Ok().json(result);
}

#[get("")]
pub async fn list(params: web::Query<ArticleListParams>) -> impl Responder {
    let result = ArticleService::list(params.0).await;
    return HttpResponse::Ok().json(result);
}

#[get("/{id}")]
pub async fn get(id: web::Path<i64>) -> impl Responder {
    let result = ArticleService::get(*id).await;
    return HttpResponse::Ok().json(result);
}

#[put("/{id}")]
pub async fn update(id: web::Path<i64>, article: web::Json<Article>) -> impl Responder {
    let result = ArticleService::update(*id, article.0).await;
    return HttpResponse::Ok().json(result);
}

#[delete("/{id}")]
pub async fn delete(id: web::Path<i64>) -> impl Responder {
    let result = ArticleService::delete(*id).await;
    return HttpResponse::Ok().json(result);
}