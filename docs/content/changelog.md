# Changelog

## [1.8.1] - 2025-12-17

- **Quiet Mode**: Added `--quiet` flag to suppress normal output


## [1.8.0] - 2025-11-14

- **Dry Run Mode**: Added `--dry` flag to preview content that will be sent to AI without actually sending or committing
- **Optimized Deleted File Handling**: Deleted files now only display "Deleted: filename" instead of full file content in diffs, making output cleaner and more efficient


## [1.7.3] - 2025-11-13

- Enhanced `gim ai` command:
  - `gim ai` now displays masked API key (first 8 characters + `***`)
  - `gim ai -k` without value showing current complete key

## [1.7.0] - 2025-08-01

- Added `--show-location` flag to command `config` to show config file location

## [1.6.1] - 2025-06-30

- Added binary to scoop bucket

## [1.5.0] - 2025-06-27

- Added `--reset` flag to `prompt` subcommand
  - Reset both diff and subject prompts to default
  - Remove prompt files if they exist
- Ignore deleted files content to reduce AI chat token consumption

## [1.4.0] - 2025-06-26

- Added mkdocs docs
- Added custom param `lines_limit` to halt app running when too many changes
- Added support for set 'update' block params: `gim update --max <M> --interval <V>`

## [1.3.2] - 2025-06-17

- add command to show 'ai' block

## [1.3.0] - 2025-05-22

- New `prompt` subcommand to manage AI prompt templates
  - View both diff and subject prompt templates
  - Edit prompt files with `--edit` flag
  - Support for custom editors with `--editor` option
  - Short aliases for prompt types (d/diff/diff_prompt, s/subject/subject_prompt)