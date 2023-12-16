use serde::{Deserialize, Serialize};

use super::{answer::Answer, question::Question};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    Questions(Vec<Question>),
    Question(Question),
    Answers(Vec<Answer>),
    Answer(Answer),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonResponse {
    pub error: bool,
    pub message: Option<String>,
    pub data: Option<ResponseType>,
}

impl JsonResponse {
    pub fn new(error: bool, message: Option<String>, data: Option<ResponseType>) -> Self {
        JsonResponse {
            error,
            message,
            data,
        }
    }
}
