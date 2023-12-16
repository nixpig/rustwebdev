mod error;
mod routes;
mod store;
mod types;

use routes::{answer, question};
use store::Store;
use types::response::JsonResponse;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

#[tokio::main]
async fn main() {
    // env_logger::init();

    let store = Store::new("http://postgres:p4ssw0rd@localhost:5432/rustwebdev").await;


    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("should be able to run db migration");

    let store_filter = warp::any().map(move || store.clone());

    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_q_and_a=info,warp=error".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("Content-Type")
        .allow_methods(&[Method::PUT, Method::POST, Method::DELETE, Method::GET]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(question::get_questions_handler)
        .with(warp::trace(|info| 
            tracing::info_span!("get_questions request", method = %info.method(), path = %info.path(), id = %uuid::Uuid::new_v4())
        ));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(question::add_question_handler);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(question::update_question_handler);

    let get_question_by_id = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(question::get_question_by_id_handler);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(question::delete_question_handler);

    let get_answers = warp::get()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(answer::get_answers_handler);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(answer::add_answer_handler);

    let get_answer_by_id = warp::get()
        .and(warp::path("answers"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(answer::get_answer_by_id_handler);

    let update_answer = warp::put()
        .and(warp::path("answer"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(answer::update_answer_handler);

    let delete_answer = warp::delete()
        .and(warp::path("answer"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(answer::delete_answer_handler);

    let get_answers_for_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(answer::get_answers_for_question_handler);

    // TODO: generate unique (incremented?) id when adding a question

    // TODO: generate a unique (incremented?) id when adding an answer

    let routes = get_questions
        .or(get_question_by_id)
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(get_answers)
        .or(get_answer_by_id)
        .or(update_answer)
        .or(add_answer)
        .or(delete_answer)
        .or(get_answers_for_question)
        .with(cors)
        .with(warp::trace::request())
        .recover(error::error_handler);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
