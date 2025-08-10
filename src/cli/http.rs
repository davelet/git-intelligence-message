use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, validator::Validate)]
struct Message {
    #[validate(length(min = 1))]
    role: String,
    #[validate(length(min = 1))]
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    stream: bool,
    extra_body: RequestExtraBody,
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
struct RequestExtraBody {
    enable_thinking: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    choices: Option<Vec<Choice>>,
    error: Option<ResponseError>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseError {
    message: String,
    r#type: Option<String>,
}

/// Sends a chat request to the specified AI API endpoint and returns the response.
///
/// # Arguments
///
/// * `url` - The API endpoint URL.
/// * `model_name` - The name of the AI model to use.
/// * `api_key` - The API key for authentication.
/// * `system` - Optional system prompt.
/// * `user` - The user input or prompt.
/// * `log_info` - Whether to print verbose log information.
///
/// # Returns
///
/// * `Ok(String)` containing the AI response if successful.
/// * `Err(Box<dyn Error>)` if the request fails or the response is invalid.
pub async fn chat(
    url: String,
    model_name: String,
    api_key: String,
    system: Option<String>,
    user: String,
    log_info: bool,
) -> Result<String, Box<dyn Error>> {
    let mut request_body = Request {
        model: model_name,
        messages: vec![Message {
            role: "user".to_string(),
            content: user,
        }],
        ..Default::default()
    };
    if let Some(system) = system {
        request_body.messages.push(Message {
            role: "system".to_string(),
            content: system,
        });
    }
    // let url = check_url(&url, &request_body.model);
    let mut url = url;
    if !url.starts_with("http") {
        if let Some(str) = get_url_by_model(&request_body.model) {
            url = str;
        } else {
            eprintln!("Error: please setup ai url first");
            std::process::exit(1);
        }
    } else {
        // If user provided a base URL, construct the full URL
        url = construct_full_url(&url);
    }

    if log_info {
        println!("ai request url: {}", url);
    }

    // 发送请求
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", &api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;
    let status = response.status();
    if status.as_u16() >= 400 {
        return Err(format!("ai request failed: {}", status).into());
    }

    let res_text = response.text().await?;
    if log_info {
        println!("ai request result ({}): {}", status, res_text);
    }

    let res: Response = serde_json::from_str(&res_text)?;

    if let Some(res) = res.choices {
        return Ok(res[0].message.content.clone());
    }
    eprintln!("{:?}", res);
    if let Some(res) = res.error {
        return Err(res.message.into());
    }
    Err("unkown exception".into())
}

/// Returns the default API URL for the given model name, if recognized.
///
/// # Arguments
///
/// * `model_name` - The name of the AI model.
///
/// # Returns
///
/// * `Some(String)` containing the default URL if the model is recognized.
/// * `None` if the model is not recognized.
pub fn get_url_by_model(model_name: &str) -> Option<String> {
    if model_name.starts_with("moonshot") {
        return Some(format!("{}{}", crate::constants::MOONSHOT_BASE_URL, crate::constants::DEFAULT_CHAT_COMPLETIONS_PATH));
    }
    if model_name.starts_with("qwen") {
        return Some(format!("{}{}", crate::constants::QWEN_BASE_URL, crate::constants::QWEN_CHAT_COMPLETIONS_PATH));
    }
    if model_name.starts_with("gpt") {
        return Some(format!("{}{}", crate::constants::GPT_BASE_URL, crate::constants::DEFAULT_CHAT_COMPLETIONS_PATH));
    }
    if model_name.starts_with("gemini") {
        return Some(format!("{}{}", crate::constants::GEMINI_BASE_URL, crate::constants::GEMINI_CHAT_COMPLETIONS_PATH));
    }
    if model_name.starts_with("doubao") {
        return Some(format!("{}{}", crate::constants::DOUBAO_BASE_URL, crate::constants::DOUBAO_CHAT_COMPLETIONS_PATH));
    }
    if model_name.starts_with("glm") {
        return Some(format!("{}{}", crate::constants::GLM_BASE_URL, crate::constants::GLM_CHAT_COMPLETIONS_PATH));
    }
    if model_name.starts_with("deepseek") {
        return Some(format!("{}{}", crate::constants::DEEPSEEK_BASE_URL, crate::constants::DEFAULT_CHAT_COMPLETIONS_PATH));
    }
    if model_name.starts_with("qianfan") {
        return Some(format!("{}{}", crate::constants::QIANFAN_BASE_URL, crate::constants::QIANFAN_CHAT_COMPLETIONS_PATH));
    }
    None
}

/// Constructs a full URL by combining a base URL with the default chat completions path.
/// Handles various cases where the base URL may already include parts of the path.
///
/// # Arguments
///
/// * `base_url` - The base URL provided by the user.
///
/// # Returns
///
/// * `String` containing the full URL.
pub fn construct_full_url(base_url: &str) -> String {
    // Remove trailing slash from base URL if present
    let trimmed_base = base_url.trim_end_matches('/');
    
    // Check if the base URL already contains the full chat completions path
    if trimmed_base.ends_with("/v1/chat/completions") {
        return trimmed_base.to_string();
    }
    
    // Check if the base URL ends with /v1, in which case we only need to add /chat/completions
    if trimmed_base.ends_with("/v1") {
        return format!("{}{}", trimmed_base, "/chat/completions");
    }
    
    // Add the default path if not already present
    format!("{}{}", trimmed_base, crate::constants::DEFAULT_CHAT_COMPLETIONS_PATH)
}

#[cfg(test)]
mod tests {
    use crate::cli::http::{chat, construct_full_url};

    #[tokio::test]
    async fn test_chat_success() {
        let result = chat(
            format!("{}{}", crate::constants::QWEN_BASE_URL, crate::constants::QWEN_CHAT_COMPLETIONS_PATH),
            "qwen2.5-0.5b-instruct".into(),
            "sk-".into(),
            Some("You are a helpful assistant.".to_string()),
            "讲个笑话".into(),
            false,
        )
        .await;
        if result.is_err() {
            println!("模型报错: {}", result.unwrap_err());
        } else {
            println!("模型回复: {}", result.unwrap());
        }
    }

    #[test]
    fn test_construct_full_url() {
        // Test with base URL without any path
        assert_eq!(
            construct_full_url("https://api.openai.com"),
            "https://api.openai.com/v1/chat/completions"
        );
        
        // Test with base URL ending with /v1
        assert_eq!(
            construct_full_url("https://api.openai.com/v1"),
            "https://api.openai.com/v1/chat/completions"
        );
        
        // Test with base URL already containing the full path
        assert_eq!(
            construct_full_url("https://api.openai.com/v1/chat/completions"),
            "https://api.openai.com/v1/chat/completions"
        );
        
        // Test with base URL ending with trailing slash
        assert_eq!(
            construct_full_url("https://api.openai.com/"),
            "https://api.openai.com/v1/chat/completions"
        );
        
        // Test with base URL ending with /v1 and trailing slash
        assert_eq!(
            construct_full_url("https://api.openai.com/v1/"),
            "https://api.openai.com/v1/chat/completions"
        );
    }
}
