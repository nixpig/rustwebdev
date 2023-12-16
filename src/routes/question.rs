use std::collections::HashMap;
use warp;
use warp::http::StatusCode;

use crate::{
    error::Error,
    store::Store,
    types::{
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, Question},
        response::{JsonResponse, ResponseType},
    },
};

pub async fn get_questions_handler(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = extract_pagination(params)?;
    }

    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(questions) => questions,
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };

    Ok(warp::reply::json(&JsonResponse::new(
        false,
        Some("found questions".to_string()),
        Some(ResponseType::Questions(res)),
    )))
}

pub async fn add_question_handler(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let client = reqwest::Client::new();

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "2ESAPL82dO2MpLuJ8HXBN3Y0yfG4ayaq")
        .body(new_question.content.clone())
        .send()
        .await
        .map_err(Error::ExternalApiError)?;

    match res.error_for_status() {
        Ok(res) => {
            let res = res.text().await.map_err(Error::ExternalApiError)?;
            println!("cleaned: {:#?}", res);

            match store.add_question(new_question).await {
                Ok(question) => Ok(warp::reply::with_status(
                    warp::reply::json(&JsonResponse::new(
                        false,
                        Some("question added".to_string()),
                        Some(ResponseType::Question(question)),
                    )),
                    StatusCode::OK,
                )),
                Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
            }
        }
        Err(e) => Err(warp::reject::custom(Error::ExternalApiError(e))),
    }
}

pub async fn update_question_handler(
    question_id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.update_question(question, question_id).await {
        Ok(question) => Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(
                false,
                Some("updated question".to_string()),
                Some(ResponseType::Question(question)),
            )),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}

pub async fn get_question_by_id_handler(
    question_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_question_by_id(question_id).await {
        Ok(question) => Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(
                false,
                Some("got question".to_string()),
                Some(ResponseType::Question(question)),
            )),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}

pub async fn delete_question_handler(
    question_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(question_id).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&JsonResponse::new(
                false,
                Some("deleted question".to_string()),
                None,
            )),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}
