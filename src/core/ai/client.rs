use std::error::Error;

use crate::config::urls;
use crate::core::ai::types::{Message, Request, Response};
use crate::utils::output;

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
        model: model_name.clone(),
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
    let mut url = url;
    if !url.starts_with("http") {
        if let Some(str) = get_url_by_model(&model_name) {
            url = str;
        } else {
            eprintln!("Error: please setup ai url first");
            std::process::exit(1);
        }
    }

    if log_info {
        output::print_normal(&format!("ai request url: {}", url));
    }

    // Send request
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
        output::print_verbose(&format!("ai request result ({}): {}", status, res_text));
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
        return Some(urls::MONOSHOT_URL.to_string());
    }
    if model_name.starts_with("qwen") {
        return Some(urls::QWEN_URL.to_string());
    }
    if model_name.starts_with("gpt") {
        return Some(urls::GPT_URL.to_string());
    }
    if model_name.starts_with("gemini") {
        return Some(urls::GEMINI_URL.to_string());
    }
    if model_name.starts_with("doubao") {
        return Some(urls::DOUBAO_URL.to_string());
    }
    if model_name.starts_with("glm") {
        return Some(urls::GLM_URL.to_string());
    }
    if model_name.starts_with("deepseek") {
        return Some(urls::DEEPSEEK_URL.to_string());
    }
    if model_name.starts_with("qianfan") {
        return Some(urls::QIANFAN_URL.to_string());
    }
    None
}
