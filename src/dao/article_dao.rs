use crate::client;
use crate::entity::article::*;
use crate::entity::page::*;
use rocket::request::Form;
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

    pub fn list(params: Form<ArticleListParams>) -> Result<Page<Article>, String> {
        let mut em = client::connect();
        let (sql, count_sql, sql_param) = ArticleDao::build_list_sql(&params.0);
        let article_result: Result<Vec<Article>, DbError> =
            em.execute_sql_with_return(&sql, &sql_param);
        let count: Result<Count, DbError> = em.execute_sql_with_one_return(&count_sql, &sql_param);
        let mut result: Page<Article> = Page {
            total: 0,
            size: params.0.size,
            data: vec![],
        };
        match article_result {
            Err(err) => {
                println!("{:?}", err);
                return Err("db error".to_string());
            }
            Ok(data) => result.data = data,
        }
        match count {
            Err(err) => {
                println!("{:?}", err);
                return Err("db error".to_string());
            }
            Ok(total) => result.total = total.count,
        }
        Ok(result)
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

        let s = prefix_sql + &sql.join(",") + &format!(" WHERE id = ${}", order + 1) + suffix_sql;
        (s, params)
    }

    fn build_list_sql<'a>(
        params: &'a ArticleListParams,
    ) -> (String, String, Vec<&'a dyn rustorm::ToValue>) {
        let mut sql = "SELECT * FROM article".to_string();
        let mut midd_sql: Vec<String> = vec![];
        let mut sql_params: Vec<&dyn rustorm::ToValue> = vec![];
        let mut order = 0;
        let mut count_sql = "SELECT count(*) as count FROM article".to_string();

        if params.title != None {
            order += 1;
            midd_sql.push(format!("title = ${}", order));
            sql_params.push(&params.title);
        };

        if params.describe != None {
            order += 1;
            midd_sql.push(format!("describe = ${}", order));
            sql_params.push(&params.describe);
        };

        if params.content != None {
            order += 1;
            midd_sql.push(format!("content = ${}", order));
            sql_params.push(&params.content);
        };

        if params.article_type != None {
            order += 1;
            midd_sql.push(format!("article_type = ${}", order));
            sql_params.push(&params.article_type);
        };

        let offset = (params.page - 1) * params.size;
        sql = if midd_sql.len() != 0 {
            sql + " WHERE"
                + &midd_sql.join("and")
                + &format!(" LIMIT {} OFFSET {}", params.size, offset)
        } else {
            sql + &format!(" LIMIT {} OFFSET {}", params.size, offset)
        };
        count_sql = if midd_sql.len() != 0 {
            count_sql + " WHERE" + &midd_sql.join("and")
        } else {
            count_sql
        };

        return (sql, count_sql, sql_params);
    }
}
