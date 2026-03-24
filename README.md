# Discord Bot

一個以 Rust 打造的 Discord 機器人，使用 [Poise](https://github.com/serenity-rs/poise) 作為指令框架、[Serenity](https://github.com/serenity-rs/serenity) 作為 Discord API 函式庫，並以 Tokio 非同步執行。

## 功能

目前已實作的 Slash Commands：

- `/help`：顯示指令說明
- `/ping`：回傳機器人延遲（ms）
- `/clear`：批次刪除訊息（需要 `MANAGE_MESSAGES` 權限）
- `/nuke`：重建目前頻道（需要 `MANAGE_CHANNELS` 權限）

## 技術棧

- Rust（Edition 2024）
- [poise](https://crates.io/crates/poise) `0.6.1`
- [tokio](https://crates.io/crates/tokio) `1.50.0`
- [tracing-subscriber](https://crates.io/crates/tracing-subscriber) `0.3.23`
- [log](https://crates.io/crates/log) `0.4.29`

## 專案結構

- `src/main.rs`：機器人啟動、框架設定、全域 hook、錯誤處理
- `src/commands.rs`：Slash Commands 實作
- `Cargo.toml`：套件資訊與依賴

## 需求環境

- 已安裝 Rust（建議使用最新版 stable）
- 一個 Discord Bot Token

## 快速開始

1. 安裝相依套件並編譯
2. 設定環境變數 `DISCORD_TOKEN`
3. 啟動機器人

### 設定環境變數

此專案啟動時會讀取 `DISCORD_TOKEN`。

若未設定，程式會在啟動時中止並提示：

- `Missing DISCORD_TOKEN env var, see README for more information.`

在 PowerShell 中可先設定：

- 當前終端有效：`$env:DISCORD_TOKEN="你的 Bot Token"`
- 永久（使用者層級）：`setx DISCORD_TOKEN "你的 Bot Token"`

> 設定後若使用 `setx`，請重新開啟終端機再啟動程式。

## 執行與測試

- 啟動：`cargo run`
- 編譯：`cargo build`
- 測試：`cargo nextest run`

## 權限與注意事項

- `/clear` 需要 `MANAGE_MESSAGES`
- `/nuke` 需要 `MANAGE_CHANNELS`
- `/nuke` 會刪除目前頻道並以相近設定建立新頻道，請謹慎使用

## 日誌

專案使用 `tracing-subscriber` 初始化輸出，可搭配 `RUST_LOG` 觀察更詳細日誌，例如：

- `RUST_LOG=info`
- `RUST_LOG=debug`

## 授權

[gpl-3.0](LICENSE.txt)
