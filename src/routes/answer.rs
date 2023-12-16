use crate::{
    error::Error,
    store::Store,
    types::{
        answer::{Answer, NewAnswer},
        response::{JsonResponse, ResponseType},
    },
};
use warp;
use warp::http::StatusCode;

pub async fn add_answer_handler(
    store: Store,
    answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_question_by_id(answer.question_id.0).await {
        Ok(_) => match store.add_answer(answer.question_id.0, answer).await {
            Ok(answer) => Ok(warp::reply::with_status(
                warp::reply::json(&JsonResponse::new(
                    false,
                    Some("added answer to question".to_string()),
                    Some(ResponseType::Answer(answer)),
                )),
                StatusCode::OK,
            )),
            Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
        },
        Err(_) => Err(warp::reject::custom(Error::ItemNotFound(
            "no question with provided id".to_string(),
        ))),
    }
}

pub async fn get_answers_handler(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_answers().await {
        Ok(answers) => Ok(warp::reply::json(&JsonResponse::new(
            false,
            Some("got answers".to_string()),
            Some(ResponseType::Answers(answers)),
        ))),

        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}

pub async fn get_answer_by_id_handler(
    answer_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_answer_by_id(answer_id).await {
        Ok(answer) => Ok(warp::reply::json(&JsonResponse::new(
            false,
            Some("got answer".to_string()),
            Some(ResponseType::Answer(answer.clone())),
        ))),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}

pub async fn update_answer_handler(
    answer_id: i32,
    store: Store,
    answer: Answer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.update_answer(answer, answer_id).await {
        Ok(answer) => Ok(warp::reply::json(&JsonResponse::new(
            false,
            Some("answer updated".to_string()),
            Some(ResponseType::Answer(answer)),
        ))),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}

pub async fn delete_answer_handler(
    answer_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_answer(answer_id).await {
        Ok(_) => Ok(warp::reply::json(&JsonResponse::new(
            false,
            Some("deleted answer".to_string()),
            None,
        ))),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}

pub async fn get_answers_for_question_handler(
    question_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_answers_for_question(question_id).await {
        Ok(answers) => Ok(warp::reply::json(&JsonResponse::new(
            false,
            Some("found answers to question".to_string()),
            Some(ResponseType::Answers(answers)),
        ))),
        Err(e) => Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}
