use std::fmt;
use std::error;

#[derive(Serialize, Debug, Clone)]
pub struct ArticleError {
    pub code: i64,
    pub message: String
}

impl ArticleError {
    pub fn from_error(err: &dyn error::Error) -> ArticleError {
        ArticleError{
            code: 500,
            message: err.to_string()
        }
    }

    pub fn from_string(err_msg: String) -> ArticleError {
        ArticleError{
            code: 500,
            message: err_msg
        }
    }
}

impl fmt::Display for ArticleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.message)
    }
}

