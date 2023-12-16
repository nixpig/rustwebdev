use reqwest::Error as ReqwestError;
use sqlx::Error as SqlxError;
use std::{fmt::Display, num::ParseIntError};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
};

use crate::JsonResponse;

#[derive(Debug)]
struct InvalidId;

impl Reject for InvalidId {}

#[derive(Debug)]
pub enum Error {
    Parse(ParseIntError),
    MissingParameters(String),
    OutOfRange(String),
    ItemNotFound(String),
    DuplicateId(String),
    DatabaseQueryError(SqlxError),
    ExternalApiError(ReqwestError),
}

impl Reject for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Parse(ref parameter_name) => {
                write!(f, "could not parse provided parameter: {}", parameter_name)
            }
            Error::MissingParameters(ref parameter_name) => {
                write!(f, "required parameter missing: {}", parameter_name)
            }
            Error::OutOfRange(ref parameter_name) => {
                write!(
                    f,
                    "value provided for parameter out of range: {}",
                    parameter_name
                )
            }
            Error::ItemNotFound(ref item_id) => {
                write!(f, "question not found: {}", item_id)
            }
            Error::DuplicateId(ref item_id) => {
                write!(f, "duplicate id: {}", item_id)
            }
            Error::DatabaseQueryError(ref err) => {
                write!(f, "database query could not be executed: {}", err)
            }
            Error::ExternalApiError(ref err) => {
                write!(f, "error querying external API: {}", err)
            }
        }
    }
}

pub async fn error_handler(r: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(true, Some(error.to_string()), None)),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(crate::error::Error::ExternalApiError(e)) = r.find() {
        Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(true, Some(e.to_string()), None)),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(_invalid_id) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(
                true,
                Some("no valid id provided".to_string()),
                None,
            )),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(true, Some(error.to_string()), None)),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(true, Some(error.to_string()), None)),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(
                true,
                Some("not found".to_string()),
                None,
            )),
            StatusCode::NOT_FOUND,
        ))
    }
}
