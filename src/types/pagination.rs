use std::collections::HashMap;

use crate::error::Error;

#[derive(Debug)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            limit: None,
            offset: 0,
        }
    }
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::Parse)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::Parse)?,
        });
    };

    Err(Error::MissingParameters(
        "start and/or end parameters missing".to_string(),
    ))
}
