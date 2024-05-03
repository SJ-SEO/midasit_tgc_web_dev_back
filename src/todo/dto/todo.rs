use serde::{Deserialize, Serialize};

use super::request::CreateTodoRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoDto {
    pub id: i32,
    pub content: String,
    pub done: bool,
}

impl TodoDto {
    pub fn new(id: i32, content: String, done: bool) -> Self {
        Self { id, content, done }
    }

    pub fn new_from_request(rq: CreateTodoRequest) -> Self {
        Self {
            id: 0,
            content: rq.content.clone(),
            done: false,
        }
    }
}
