use actix_web::{error, http::StatusCode, HttpResponse};
use pytube_wrpr::Error as PError;
use serde::Serialize;
use std::fmt;

use actix_web::Error as ActixError;

#[derive(Debug, Serialize, Clone)]
pub enum YtDlErrors {
    ActixError(String),
    DlError(String),
    FileError(String),
}
impl YtDlErrors {
    fn err_response(&self) -> String {
        match self {
            YtDlErrors::ActixError(msg) => {
                println!("Server error occured. {:?}", msg);
                "Server error".to_string()
            }
            YtDlErrors::DlError(msg) => {
                println!("Download error occured. {:?}", msg);
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
            YtDlErrors::DlError(_) | YtDlErrors::FileError(_) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(self.err_response())
    }
}
impl fmt::Display for YtDlErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.err_response())
    }
}
impl From<ActixError> for YtDlErrors {
    fn from(err: ActixError) -> Self {
        YtDlErrors::ActixError(err.to_string())
    }
}
impl From<PError> for YtDlErrors {
    fn from(err: PError) -> Self {
        YtDlErrors::DlError(err.to_string())
    }
}
