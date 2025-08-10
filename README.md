# Git Intelligence Message (GIM) 🚀

[![Crates.io](https://img.shields.io/crates/v/git-intelligence-message)](https://crates.io/crates/git-intelligence-message)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An advanced Git commit message generation utility designed to automatically craft high-quality commit messages with precision and sophistication.

Static site: https://git-intelligence-message.pages.dev/ 

This tool is still in its early stages. I am working on adding more features and fixing bugs. I am also working on adding more tests to ensure that the tool is reliable.

If you find any issues, please report them to me. I will do my best to fix them as soon as possible.

I look forward to hearing your feedback. Feel free to fire feature request or bug issue if you want and necessary: [Issues](https://github.com/davelet/git-intelligence-message/issues/new).

## Platform Support

This project now officially supports multiple platforms:
- Windows (7 and later)
- macOS (10.12 and later)
- Linux (most distributions)

The application has been tested on these platforms and should work without issues. If you encounter any platform-specific problems, please [report them](https://github.com/davelet/git-intelligence-message/issues/new).

## AI Provider URL Configuration

When configuring the AI provider URL, you can now simply provide the base URL without the full path. The application will automatically append the appropriate path based on the provider:

- For OpenAI: `https://api.openai.com` (automatically becomes `https://api.openai.com/v1/chat/completions`)
- For providers with base URL ending in `/v1`: `https://api.provider.com/v1` (automatically becomes `https://api.provider.com/v1/chat/completions`)
- For providers with full URL: `https://api.provider.com/v1/chat/completions` (used as-is)

This simplifies configuration and makes it more intuitive for users.

Thank you for your interest in this project.
