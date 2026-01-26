# AI Coding Agent Instructions - Discord Bot

## Project Overview

A Rust Discord bot built with **Poise** (command framework) and **Serenity** (Discord API). Uses **Tokio** for async runtime and **tracing-subscriber** for logging. The bot architecture emphasizes modularity with commands separated into `src/commands.rs` and core setup in `src/main.rs`.

## Architecture & Data Flow

### Core Type System

```rust
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}  // User-defined data passed to all commands
```

All commands use these globally-defined types. Never define alternative Error or Context types—add fields to `Data` struct instead if state is needed.

### Framework Layers

1. **Setup Phase** (`framework.setup`): Initializes bot, logs in, registers commands globally
2. **Pre/Post Hooks**: `pre_command` logs execution start, `post_command` logs on success
3. **Error Handler** (`on_error`): Catches framework errors and command-specific errors separately
4. **Command Check**: Global predicate (`command_check`) runs before every command execution

Access Serenity context via `ctx.serenity_context()` when you need low-level Discord API calls.

## Command Patterns

### Defining Commands

```rust
#[poise::command(slash_command)]
pub async fn command_name(ctx: Context<'_>, arg: Option<String>) -> Result<(), Error> {
    ctx.say("response").await?;
    Ok(())
}
```

- Always return `Result<(), Error>`
- Use `#[description = "..."]` on parameters for slash command help
- Add `#[poise::command(...)]` attributes for permissions: `default_member_permissions = "MANAGE_MESSAGES"`

### Response Types

- **Simple text**: `ctx.say(format!("text")).await?`
- **Rich embeds**: `ctx.send(poise::CreateReply::default().embed(...)).await?`
- **Interactive components**: Use `ComponentInteractionCollector` with timeout handling
  - Register button custom IDs in filter predicate
  - Always clear components on timeout/completion to avoid "Unknown Message" errors
  - Use `.ephemeral(true)` for temporary confirmations

### Error Context

The `on_error` handler distinguishes:

- `FrameworkError::Setup` → startup failure (panic)
- `FrameworkError::Command` → command execution error (log and respond via `poise::builtins::on_error`)
- All others → forward to default handler

## Critical Implementation Details

### Message Deletion (see `clear` command)

When fetching >100 messages, make multiple `GetMessages` requests because Discord API limits 100 per call:

```rust
let msg = c.messages(ctx.serenity_context(), GetMessages::new().limit(count)).await?;
if count > 100 {
    msg.append(c.messages(..., GetMessages::new().limit(count).before(msg[0].id)).await?.as_mut());
}
```

### Channel Recreation (see `nuke` command)

Preserve all Discord channel settings during recreation:

- `available_tags`, `permissions`, `nsfw`, `position`, `kind`, `topic`
- Optional fields: `default_auto_archive_duration`, `rate_limit_per_user`, `bitrate`, `user_limit`, `rtc_region`, `video_quality_mode`
- Use `create_channel_builder()` helper to avoid duplicating this logic

### Async Context Arc Wrapping

For background operations or non-command contexts, wrap Serenity context in `std::sync::Arc`:

```rust
let serenity_context = std::sync::Arc::new(ctx.serenity_context().clone());
// Now usable outside command scope or in async closure
```

### Interaction Message Editing

Always `.await?` on message edits **after** user confirms, then fall back gracefully if "Unknown Message" errors (use `println!` to log, not panic).

## Build & Test Commands

- **Build**: `cargo build`
- **Run**: `cargo run` (requires `DISCORD_TOKEN` env var)
- **Tests**: `cargo nextest run` (VS Code task available)
- **Logging**: Set `RUST_LOG=debug` or `info` to see tracing output

## Configuration

- Bot token: Required via `DISCORD_TOKEN` environment variable
- Intents: Currently `serenity::GatewayIntents::non_privileged()` (doesn't require message content)
- Global slash commands only (no prefix commands currently)

## File Organization

- `src/main.rs`: Framework setup, hooks, error handler, entry point
- `src/commands.rs`: All command implementations (organized by functionality)
- Add new commands here and register in `framework.options().commands` vector

## Key Dependencies & Versions

- `poise = "0.6.1"` → Framework, use `poise::CreateReply`, `poise::serenity_prelude`
- `tokio = "1.49.0"` → Async runtime with multi-thread feature
- `tracing-subscriber = "0.3.22"` → Initialize with `tracing_subscriber::fmt::init()`

## Common Gotchas

1. **Message count >100**: Requires paginated requests (not a single query)
2. **Editing interactions**: May fail if message is deleted; never panic on edit errors
3. **Button timeouts**: Always update ephemeral replies on timeout to avoid stale interaction state
4. **Arc cloning Serenity context**: Required for passing context into non-command async functions
5. **Edition 2024**: This project uses 2024 edition (check Cargo.toml)—ensure trait syntax compatibility

## Logging & Debugging

- Framework logs are printed to stdout (use `println!`)
- Application logs use `tracing-subscriber` (initialize in `main()`)
- Pre/post command hooks provide execution timing
- Test with `RUST_LOG=debug cargo run` for detailed output
