use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

use actix_web::Error as ActixError;
use rustube::url::ParseError;

#[derive(Debug, Serialize)]
pub enum YtDlErrors {
    ActixError(String),
    RtError(String),
    FileError(String),
}
impl YtDlErrors {
    fn error_response(&self) -> String {
        match self {
            YtDlErrors::ActixError(msg) => {
                println!("Server error occured. {:?}", msg);
                "Server error".to_string()
            }
            YtDlErrors::RtError(msg) => {
                println!("Rustube error occured. {:?}", msg);
                "Rustube error".to_string()
            }
            YtDlErrors::FileError(msg) => {
                println!("Error deleting file(s). {:?}", msg);
                "File deletion error".to_string()
            }
        }
    }
}
impl error::ResponseError for YtDlErrors {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            YtDlErrors::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            YtDlErrors::RtError(_) | YtDlErrors::FileError(_) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(self.error_response())
    }
}
impl fmt::Display for YtDlErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}
impl From<ActixError> for YtDlErrors {
    fn from(err: ActixError) -> Self {
        YtDlErrors::ActixError(err.to_string())
    }
}
impl From<ParseError> for YtDlErrors {
    fn from(err: ParseError) -> Self {
        YtDlErrors::RtError(err.to_string())
    }
}
impl From<rustube::Error> for YtDlErrors {
    fn from(err: rustube::Error) -> Self {
        YtDlErrors::RtError(err.to_string())
    }
}
