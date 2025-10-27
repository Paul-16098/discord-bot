# Discord Bot 项目

[en](README.md) | zh-cn

此项目是一个使用 [Poise](https://docs.rs/poise/latest/poise/) 框架构建的 Discord 机器人。它利用 Serenity 库与 Discord API 交互，并使用 Tokio 作为异步运行时。该机器人旨在处理各种命令和事件，注重模块化和可维护性。

## 技术栈

- **编程语言**: Rust
- **框架**: Poise（用于命令处理）
- **库**:
  - Serenity: Discord API 交互
  - Tokio: 异步运行时
  - Tracing-subscriber: 日志记录

## 项目架构

该机器人使用 Poise 框架简化命令和事件处理。关键架构组件包括：

- **命令**: 使用 `#[poise::command]` 属性定义为异步函数。命令是模块化的，位于 `src/commands.rs`。
- **错误处理**: 自定义错误处理在 `src/main.rs` 的 `on_error` 函数中实现。
- **事件处理**: 使用框架配置中的 `event_handler` 选项记录事件。

## 快速开始

### 前置条件

- 从 [rust-lang.org](https://www.rust-lang.org/) 安装 Rust。
- 使用您的 Discord 机器人令牌设置 `DISCORD_TOKEN` 环境变量。

### 安装步骤

1. 克隆此仓库：

   ```bash
   git clone <repository-url>
   cd discord-bot
   ```

2. 构建项目：

   ```bash
   cargo build
   ```

3. 运行机器人：

   ```bash
   cargo run
   ```

## 项目结构

- **`src/main.rs`**: 应用程序的入口点。设置机器人框架、初始化命令并处理全局错误。
- **`src/commands.rs`**: 包含机器人命令的实现，例如 `help`、`ping`、`clear` 和 `nuke`。
- **`Cargo.toml`**: 定义项目依赖项，包括 `poise`、`tokio` 和 `tracing-subscriber`。

## 主要功能

- 模块化命令处理
- 高级错误和事件日志记录
- 示例命令如 `help`、`ping`、`clear` 和 `nuke`

## 开发工作流

- **构建和运行**:

  - 使用 `cargo build` 构建项目。
  - 使用 `cargo run` 运行机器人。

- **调试**:
  - 日志由 `tracing-subscriber` crate 管理。将 `RUST_LOG` 环境变量设置为所需级别（例如，`info`、`debug`）。

## 编码标准

- 命令遵循 Poise 框架的约定。
- 使用 `ctx.say` 提供用户友好的错误消息。
- 确保命令定义的模块化和可维护性。

## 贡献

- 遵循上述编码标准。
- 参考 `src/commands.rs` 中的示例，了解如何添加新命令。
- 确保所有贡献都经过测试和记录。

---

有关更多详细信息，请参阅 [Poise 文档](https://docs.rs/poise/latest/poise/)。
