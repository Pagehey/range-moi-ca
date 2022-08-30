use serde::{Serialize};

#[derive(Serialize)]
pub enum Status {
    Success,
    Failure
}

#[derive(Serialize)]
pub struct Response {
    status: Status,
    message: String
}

impl Response {
    pub fn new(status: Status, message: String) -> Self {
        return Response { status, message };
    }
}
