use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use warp::{
    filters::cors::CorsForbidden, http::Method, http::StatusCode, Filter, Rejection, Reply,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
struct QuestionId(String);

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }
}

async fn get_questions(store: Store) -> Result<impl Reply, Rejection> {
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_items = warp::get()
        .and(warp::path("questions").and(warp::path::end()))
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
