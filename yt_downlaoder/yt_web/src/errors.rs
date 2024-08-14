use actix_web::error::Error as ActErr;
use actix_web::ResponseError;
use actix_web::{error, http::StatusCode, HttpResponse};
use awc::error::SendRequestError;
use serde::Serialize;
use sqlx::Error as SqlxErr;
use tera::Error as TeraErr;

#[derive(Debug, Serialize)]
pub enum WebErrors {
    TemplErr(String),
    YtServErr(String),
    DBError(String),
    ActixErr(String),
}
impl WebErrors {
    fn error_response(&self) -> String {
        match self {
            WebErrors::TemplErr(msg) => {
                println!("Templae engine error {msg}");
                "Template Error".into()
            }
            WebErrors::YtServErr(msg) => {
                println!("Youtube_Service server error {msg}");
                "Sever Error".into()
            }
            WebErrors::DBError(msg) => {
                println!("Database error {msg}");
                "Database error".into()
            }
            WebErrors::ActixErr(msg) => {
                println!("Actix web error {msg}");
                "Actix error".into()
            }
        }
    }
}
impl error::ResponseError for WebErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            WebErrors::ActixErr(_)
            | WebErrors::DBError(_)
            | WebErrors::TemplErr(_)
            | WebErrors::YtServErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("text/html")
            .body(self.error_response())
    }
}
impl std::fmt::Display for WebErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error_response())
    }
}
impl From<SqlxErr> for WebErrors {
    fn from(err: SqlxErr) -> Self {
        WebErrors::DBError(err.to_string())
    }
}
impl From<ActErr> for WebErrors {
    fn from(err: ActErr) -> Self {
        WebErrors::ActixErr(err.to_string())
    }
}
impl From<TeraErr> for WebErrors {
    fn from(err: TeraErr) -> Self {
        WebErrors::TemplErr(err.to_string())
    }
}
impl From<SendRequestError> for WebErrors {
    fn from(err: SendRequestError) -> Self {
        WebErrors::YtServErr(err.to_string())
    }
}
