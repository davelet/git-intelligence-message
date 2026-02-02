use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Message {
    #[validate(length(min = 1))]
    pub role: String,
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub stream: bool,
    pub extra_body: RequestExtraBody,
}

impl Default for Request {
    fn default() -> Self {
        let empty_string = String::new();
        Self {
            model: empty_string,
            messages: Default::default(),
            temperature: 0.3,
            stream: false,
            extra_body: RequestExtraBody {
                enable_thinking: false,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestExtraBody {
    pub enable_thinking: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub choices: Option<Vec<Choice>>,
    pub error: Option<ResponseError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}
