use core::fmt;

use actix_web::error::Error as AcErr;
use actix_web::{error, http::StatusCode, HttpResponse};
use awc::error::SendRequestError;
use serde::Serialize;
use sqlx::Error as SqlxErr;

#[derive(Debug, Serialize)]
pub enum YtManErr {
    DBError(String),
    ConnectionError(String),
    ActixErr(String),
}
impl YtManErr {
    fn err_response(&self) -> String {
        match self {
            YtManErr::DBError(msg) => {
                println!("Database related error occured {msg}");
                "Database Error".into()
            }
            YtManErr::ConnectionError(msg) => {
                println!("Connection error, could not connect to download manager {msg}");
                "Connection error".into()
            }
            YtManErr::ActixErr(msg) => {
                println!("Sever error occuredcc {msg}");
                "Sever error".into()
            }
        }
    }
}
impl error::ResponseError for YtManErr {
    fn status_code(&self) -> StatusCode {
        match self {
            YtManErr::ConnectionError(_) | YtManErr::DBError(_) | YtManErr::ActixErr(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.err_response())
    }
}
impl std::fmt::Display for YtManErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.err_response())
    }
}
impl From<SqlxErr> for YtManErr {
    fn from(err: SqlxErr) -> Self {
        YtManErr::DBError(err.to_string())
    }
}
impl From<AcErr> for YtManErr {
    fn from(err: AcErr) -> Self {
        YtManErr::ActixErr(err.to_string())
    }
}
impl From<SendRequestError> for YtManErr {
    fn from(err: SendRequestError) -> Self {
        YtManErr::ConnectionError(err.to_string())
    }
}
