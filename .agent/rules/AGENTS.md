---
trigger: always_on
---

# AGENTS

## Purpose
This file orients coding agents working in this repo. Keep changes focused, avoid unrelated formatting, and follow existing patterns.
The repo contains the `rustlog` library which helps logging data from main application or from another library either on terminal or in a specific file.

## Repo layout
1. `src/` contains the source code for the library (`lib.rs`, `data.rs`, `log.rs`, `log_config.rs`).
2. `tests/` contains integration tests (`integration.rs`).
3. `Cargo.toml` defines the package and dependencies.

## Build & test
- Build the project: `cargo build`
- Run tests: `cargo test`

## Change guidelines
- Prefer small, targeted edits; avoid sweeping refactors unless asked.
- Keep ASCII in new content unless the file already uses non-ASCII.
- Add comments only when logic is non-obvious.
- Update or create documentation when needed. Methods and functions documentation needs to include : functionality description, parameters description, return description, error handling, panicking (when concerned).
- Update and run tests after each code update.
- If you need to touch multiple crates, explain why in the final response.
- Always ask for user review after generating a action plan. Never update code by yourself.

## Naming rules
1. Follow standard Rust naming conventions (snake_case for variables/functions, CamelCase for types/traits, SCREAMING_SNAKE_CASE for constants).

## Git rules
This rule applies each time a git branch needs to be created or renamed

### Instructions
1. Never ask for a branch name, define it by yourself
2. Analyse the task :
  - In case of a new feature : /feat/task-name
  - In case of bug fix : /fix/task-name
3. Always lower case
4. When the pull request is linked to a Github issue, add 'Closes #ID' to the pull request message.

### End of task
After each successful merge :
1. Always delete the associated branch
2. Confirm that the associated GitHub issue is closed
