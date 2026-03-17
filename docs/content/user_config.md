This feature was introduced in version `1.4.0`.

# lines-limit

`lines-limit` is an integer that limits the maximum number of lines per commit. If this limit is exceeded, the application will not execute. The default value is `1000`.

You can configure this parameter using `gim config --lines-limit <LINES_LIMIT>`.

# max-files

`max-files` is an integer that limits the maximum number of changed files to send to AI. When the number of changed files exceeds this limit, GIM will intelligently select the most significant files based on:

1. **Lines changed**: Files with more changes are prioritized
2. **File type filtering**: If code changes exceed 50% of total changes, only code files are included (filtering out config, docs, etc.)

The default value is `10`.

You can configure this parameter using:

```bash
gim config --max-files <MAX_FILES>
```

You can also override this value temporarily using the `-n` or `--max-files` CLI option:

```bash
# Limit to 5 files for this commit
gim -n 5

# Or use the long form
gim --max-files 5
```

**Supported file type classifications:**

- **Code**: `.rs`, `.go`, `.py`, `.js`, `.ts`, `.java`, `.c`, `.cpp`, etc.
- **Config**: `.xml`, `.toml`, `.yaml`, `.json`, `.ini`, `.env`, etc.
- **Doc**: `.md`, `.txt`, `.rst`, `.adoc`, etc.

# show-location

Since version `1.7.0`, you can use `--show-location` flag to show config file location.
And it opens the default file manager to the config file location.

```bash
gim config --show-location
```