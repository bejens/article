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

    pub fn get(id: i64) -> Result<Article, String> {
        let mut em = client::connect();
        let sql = "SELECT * FROM article WHERE id = $1";
        let article_result: Result<Article, DbError> = em.execute_sql_with_one_return(sql, &[&id]);
        match article_result {
            Ok(data) => Ok(data),
            Err(err) => {
                println!("{:?}", err);
                Err("db error".to_string())
            }
        }
    }

    pub fn update(article: Article) -> Result<Article, String> {
        let mut em = client::connect();
        let (sql, mut params) = ArticleDao::build_sql(&article);
        params.push(&article.id);
        let article_result: Result<Article, DbError> =
            em.execute_sql_with_one_return(&sql, &params);
        match article_result {
            Ok(data) => Ok(data),
            Err(err) => {
                println!("{:?}", err);
                Err("db error".to_string())
            }
        }
    }

    pub fn delete(id: i64) -> Result<Article, String> {
        let mut em = client::connect();
        let sql = "DELETE FROM article WHERE id = $1 RETURNING *";
        let article_result: Result<Article, DbError> = em.execute_sql_with_one_return(&sql, &[&id]);
        match article_result {
            Ok(data) => Ok(data),
            Err(err) => {
                println!("{:?}", err);
                Err("db error".to_string())
            }
        }
    }

    fn build_sql(article: &Article) -> (String, Vec<&dyn rustorm::ToValue>) {
        let mut order = 0;

        let prefix_sql = "UPDATE article SET ".to_string();
        let mut sql: Vec<String> = vec![];

        let mut params: Vec<&dyn rustorm::ToValue> = vec![];

        if article.title != "" {
            order += 1;
            sql.push(format!("title = ${}", order));
            params.push(&article.title);
        };
        if article.keyword != "" {
            order += 1;
            sql.push(format!("keyword = ${}", order));
            params.push(&article.keyword);
        };
        if article.describe != "" {
            order += 1;
            sql.push(format!("describe = ${}", order));
            params.push(&article.describe);
        };
        if article.cover != "" {
            order += 1;
            sql.push(format!("cover = ${}", order));
            params.push(&article.cover);
        };
        if article.content != "" {
            order += 1;
            sql.push(format!("content = ${}", order));
            params.push(&article.content);
        };

        let suffix_sql = " RETURNING *";

        let s = prefix_sql
            + &sql.join(",")
            + format!(" WHERE id = ${}", order + 1).as_ref()
            + suffix_sql;
        (s, params)
    }
}
