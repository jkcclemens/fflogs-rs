use net::FfLogsError;

use serde_json::Error as JsonError;
use reqwest::Error as ReqwestError;
use url::ParseError as UrlParseError;

use std::error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  FfLogs(FfLogsError),
  Json(JsonError),
  Reqwest(ReqwestError),
  Url(UrlParseError),
  UrlNotBase,
}

impl error::Error for Error {
  fn description(&self) -> &str {
    "fflogs error"
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      Error::Json(ref e) => Some(e),
      Error::Reqwest(ref e) => Some(e),
      Error::Url(ref e) => Some(e),
      _ => None
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match *self {
      Error::FfLogs(ref e) => write!(f, "fflogs error (status {}): {}", e.status, e.error),
      Error::Json(ref e) => write!(f, "{}", e),
      Error::Reqwest(ref e) => write!(f, "{}", e),
      Error::Url(ref e) => write!(f, "{}", e),
      Error::UrlNotBase => write!(f, "url could not be a base"),
    }
  }
}
