# Prompt Management

View and edit the AI prompt used for generating commit message, for description and subject respectively:

## File-based Prompt Management

```bash
# View current prompt
gim prompt

# Open the prompt files in default file manager for editing
gim prompt --edit

# Edit a specific prompt file with default editor
gim prompt --edit --prompt diff

# Edit a specific prompt file with custom editor, 
# like 'code', 'vim' or any other text editor available on your Mac
gim prompt --edit --prompt subject --editor code

# Reset both diff and subject prompts to default
# By removing prompt files if they exist
gim prompt --reset
```

The `-prompt` option can take these params:

- `d`, `diff`, `diff_prompt` for summarizing file changes, which will be used as the commit description.
- `s`, `subject`, `subject_prompt` for generating the commit subject based on the summary of file changes.

## Local Project Prompts (.gim directory)

You can create project-specific prompt files by creating a `.gim` directory in your git repository root:

```bash
# Create .gim directory in your project
mkdir .gim

# Create project-specific diff prompt
echo "Analyze changes for this project's specific needs" > .gim/diff_prompt.txt

# Create project-specific subject prompt  
echo "Generate commit messages following our team's conventions" > .gim/subject_prompt.txt
```

### File Structure

```
your-project/
├── .git/
├── .gim/
│   ├── diff_prompt.txt      # Custom diff analysis prompt
│   └── subject_prompt.txt   # Custom commit message prompt
├── src/
└── ...
```

### Benefits of .gim Directory

- **Team Consistency**: All team members use the same prompts when working on the project
- **Project-Specific**: Tailor prompts to your project's specific requirements and conventions
- **Version Control**: Include prompt files in git for consistent team usage
- **Priority**: Local `.gim` prompts override global config prompts but can be overridden by command-line arguments

## Priority Order

The prompts are used in the following priority order:

1. **Command-line arguments** (`--diff-prompt` / `--subject-prompt`) - Temporary override for single use
2. **Local `.gim` directory** - Project-specific prompt files (if they exist)
3. **Config directory** - Global prompt files managed with `gim prompt`
4. **Built-in defaults** - Default prompts included with the tool

This allows you to:
- Set up project-specific prompts in a `.gim` directory for consistent team usage
- Override them temporarily with command-line arguments for special cases
- Fall back to global prompts when no project-specific ones exist
- Use built-in defaults as the final fallback

## Command-line Custom Prompts

You can also override prompts temporarily using command-line arguments without modifying files:

```bash
# Use custom diff prompt for a single commit
gim --diff-prompt "Summarize each file change focusing on security implications"

# Use custom subject prompt for a single commit  
gim --subject-prompt "Generate a concise commit message following conventional commit format"

# Use both custom prompts together
gim --diff-prompt "Analyze changes for performance impact" --subject-prompt "Create performance-focused commit message"

# Combine with other options
gim -a --diff-prompt "Review changes for breaking changes" --subject-prompt "Generate breaking change commit"

# Test custom prompts with dry run
gim --dry --diff-prompt "Test prompt" --subject-prompt "Test subject"
```

