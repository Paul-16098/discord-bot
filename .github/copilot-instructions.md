# Copilot Instructions for Discord Bot Project

## Project Overview

This project is a Discord bot built using the [Poise](https://docs.rs/poise/latest/poise/) framework. It leverages the Serenity library for Discord API interactions and uses Tokio for asynchronous runtime. The bot is designed to handle various commands and events, with a focus on modularity and maintainability.

### Key Components

- **`src/main.rs`**: Entry point of the application. Sets up the bot framework, initializes commands, and handles global error handling.
- **`src/commands.rs`**: Contains the implementation of bot commands, such as `help`, `ping`, `clear`, and `nuke`.
- **`Cargo.toml`**: Defines project dependencies, including `poise`, `tokio`, and `tracing-subscriber`.

## Architecture

- **Framework**: The bot uses the Poise framework, which simplifies command and event handling.
- **Commands**: Each command is defined as an asynchronous function with the `#[poise::command]` attribute. Commands are modular and can be easily added to the `commands` module.
- **Error Handling**: Custom error handling is implemented in the `on_error` function in `src/main.rs`. It distinguishes between setup errors, command errors, and other framework errors.
- **Event Handling**: Events are logged using the `event_handler` option in the framework configuration.

## Developer Workflows

### Building and Running

1. Ensure you have the `DISCORD_TOKEN` environment variable set.
2. Build the project:
   ```bash
   cargo build
   ```
3. Run the bot:
   ```bash
   cargo run
   ```

### Testing

- Use the `ast-grep: test` task to run interactive tests:
  ```bash
  sg test --interactive
  ```
- Use the `ast-grep: scan` task to scan the codebase:
  ```bash
  sg scan
  ```

### Debugging

- Logs are managed using the `tracing-subscriber` crate. Ensure `RUST_LOG` is set to the desired level (e.g., `info`, `debug`).

## Project-Specific Conventions

- **Command Structure**: Commands are defined in `src/commands.rs` and follow the Poise framework's conventions. Each command function:
  - Uses the `Context` type for accessing Discord and framework data.
  - Returns a `Result<(), Error>`.
- **Error Messages**: Use `ctx.say` to send user-friendly error messages to Discord channels.
- **Channel Management**: The `nuke` command demonstrates advanced channel management, including cloning channel properties and recreating channels.

## Integration Points

- **Environment Variables**: The bot requires a `DISCORD_TOKEN` environment variable to authenticate with Discord.
- **External Libraries**:
  - `poise`: Command framework.
  - `tokio`: Asynchronous runtime.
  - `tracing-subscriber`: Logging.

## Examples

### Adding a New Command

To add a new command, define it in `src/commands.rs`:

```rust
#[poise::command(slash_command)]
pub async fn new_command(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This is a new command!").await?;
    Ok(())
}
```

Then, register the command in `src/main.rs`:

```rust
commands: vec![commands::help(), commands::ping(), commands::clear(), commands::nuke(), commands::new_command()],
```

### Handling Errors

Customize error handling in the `on_error` function in `src/main.rs`:

```rust
match error {
    poise::FrameworkError::Command { error, ctx, .. } => {
        println!("Error in command `{}`: {:?}", ctx.command().name, error);
    }
    _ => {}
}
```

---

For more details, refer to the [Poise documentation](https://docs.rs/poise/latest/poise/).
