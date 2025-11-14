# AI Configuration

Before generating git commit message, you have to setup your ai model.

## Configuration

Utilise the `gim ai` command to configure AI-related parameters:

```bash
# Configure AI model
gim ai --model "your-model-name"

# Set API key
gim ai --apikey "your-api-key"
# if you didn't provide any text, it outputs current api key:
gim ai -k

# Define API endpoint
gim ai --url "your-api-url"

# Set output language
gim ai --language "your-language"

# print current configuration (with masked api key)
gim ai
```

> Important: The `--url` parameter only supports OpenAI-compatible API endpoints ,such as OpenAI official or third-party services compatible with OpenAI protocol.

## AI Configuration Options

- `-m, --model <STRING>`: Specify the AI model to be utilised
- `-k, --apikey <STRING>`: Configure the API key for AI service
- `-u, --url <STRING>`: (Optional) Set the API endpoint for AI service. It's optional if your model matches built-in models prefixes described below.
- `-l, --language <STRING>`: (Optional) Define the language of generated commit messages. It's optional as 'English' is default.

> `gim ai -h` is available to find help message

## Built-in Models Support

The following model prefixes are supported with their respective default endpoints:

| Model Prefix   | Service Provider | Default Endpoint |
|----------------|------------------|------------------|
| `gpt-*`       | OpenAI           | `https://api.openai.com/v1/chat/completions` |
| `moonshot-*`  | Moonshot AI      | `https://api.moonshot.cn/v1/chat/completions` |
| `qwen-*`      | Alibaba Qwen     | `https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions` |
| `gemini-*`    | Google Gemini    | `https://generativelanguage.googleapis.com/v1beta/openai/` |
| `doubao-*`    | ByteDance Doubao | `https://ark.cn-beijing.volces.com/api/v3/chat/completions` |
| `glm-*`       | THUDM GLM        | `https://open.bigmodel.cn/api/paas/v4/chat/completions` |
| `deepseek-*`  | DeepSeek         | `https://api.deepseek.com/chat/completions` |
| `qianfan-*`   | Baidu Qianfan    | `https://qianfan.baidubce.com/v2/chat/completions` |

You can use any model name starting with these prefixes, and the corresponding endpoint will be used automatically (so you don't need to set `--url`).
