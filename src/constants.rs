pub const REPOSITORY: &str = "git-intelligence-message";

pub const DIFF_PROMPT_FILE: &str = "diff_prompt.txt";
pub const SUBJECT_PROMPT_FILE: &str = "subject_prompt.txt";

// Base URLs for different AI providers
pub const MOONSHOT_BASE_URL: &str = "https://api.moonshot.cn";
pub const QWEN_BASE_URL: &str = "https://dashscope.aliyuncs.com";
pub const GPT_BASE_URL: &str = "https://api.openai.com";
pub const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com";
pub const DOUBAO_BASE_URL: &str = "https://ark.cn-beijing.volces.com";
pub const GLM_BASE_URL: &str = "https://open.bigmodel.cn";
pub const DEEPSEEK_BASE_URL: &str = "https://api.deepseek.com";
pub const QIANFAN_BASE_URL: &str = "https://qianfan.baidubce.com";

// Default paths for different AI providers
pub const DEFAULT_CHAT_COMPLETIONS_PATH: &str = "/v1/chat/completions";
pub const QWEN_CHAT_COMPLETIONS_PATH: &str = "/compatible-mode/v1/chat/completions";
pub const GEMINI_CHAT_COMPLETIONS_PATH: &str = "/v1beta/openai/";
pub const DOUBAO_CHAT_COMPLETIONS_PATH: &str = "/api/v3/chat/completions";
pub const GLM_CHAT_COMPLETIONS_PATH: &str = "/api/paas/v4/chat/completions";
pub const QIANFAN_CHAT_COMPLETIONS_PATH: &str = "/v2/chat/completions";

pub const CUSTOM_SECTION_NAME: &str = "user";
pub const DIFF_SIZE_LIMIT: usize = 1000;
