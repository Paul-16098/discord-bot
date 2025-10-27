# Discord Bot Project

en | [zh-cn](README.zh-cn.md)

This project is a Discord bot built using the [Poise](https://docs.rs/poise/latest/poise/) framework. It leverages the Serenity library for Discord API interactions and uses Tokio for asynchronous runtime. The bot is designed to handle various commands and events, with a focus on modularity and maintainability.

## Technology Stack

- **Programming Language**: Rust
- **Framework**: Poise (for command handling)
- **Libraries**:
  - Serenity: Discord API interactions
  - Tokio: Asynchronous runtime
  - Tracing-subscriber: Logging

## Project Architecture

The bot uses the Poise framework to simplify command and event handling. Key architectural components include:

- **Commands**: Defined as asynchronous functions with the `#[poise::command]` attribute. Commands are modular and located in `src/commands.rs`.
- **Error Handling**: Custom error handling is implemented in the `on_error` function in `src/main.rs`.
- **Event Handling**: Events are logged using the `event_handler` option in the framework configuration.

## Getting Started

### Prerequisites

- Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- Set up the `DISCORD_TOKEN` environment variable with your Discord bot token.

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd discord-bot
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the bot:

   ```bash
   cargo run
   ```

## Project Structure

- **`src/main.rs`**: Entry point of the application. Sets up the bot framework, initializes commands, and handles global error handling.
- **`src/commands.rs`**: Contains the implementation of bot commands, such as `help`, `ping`, `clear`, and `nuke`.
- **`Cargo.toml`**: Defines project dependencies, including `poise`, `tokio`, and `tracing-subscriber`.

## Key Features

- Modular command handling
- Advanced error and event logging
- Example commands like `help`, `ping`, `clear`, and `nuke`

## Development Workflow

- **Building and Running**:
  - Build the project with `cargo build`.
  - Run the bot with `cargo run`.
- **Testing**:

  - Use the `ast-grep: test` task to run interactive tests:

    ```bash
    sg test --interactive
    ```

  - Use the `ast-grep: scan` task to scan the codebase:

    ```bash
    sg scan
    ```

- **Debugging**:
  - Logs are managed using the `tracing-subscriber` crate. Set the `RUST_LOG` environment variable to the desired level (e.g., `info`, `debug`).

## Coding Standards

- Commands follow the Poise framework's conventions.
- Use `ctx.say` for user-friendly error messages.
- Ensure modularity and maintainability in command definitions.

## Testing

- Testing is integrated into the development workflow using `ast-grep` tasks.
- Ensure all commands and features are thoroughly tested before deployment.

## Contributing

- Follow the coding standards outlined above.
- Refer to the examples in `src/commands.rs` for guidance on adding new commands.
- Ensure all contributions are tested and documented.

---

For more details, refer to the [Poise documentation](https://docs.rs/poise/latest/poise/).
