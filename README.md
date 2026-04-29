# g0v Summit 2026 報到 Registration Dashboard

This project is implemented with Leptos, a fullstack Rust framework. It's intended to be an application for g0v Summit 2026 registration desk.
g0v Summit 2026 will issue VCs and verify attendee registration with moda's [Ditigal Wallet](https://wallet.gov.tw/zh-tw), the registration dashboard is expected to be used only when manual registration is necessary. It syncs registration data between both digital wallet verification and manual registration (not yet implemented). For now it's demo only and a large portion of the codebase is generated and modified with the help of Codex.

此專案會作為 g0v Summit 2026 報到資料後臺，g0v Summit 2026 將使用數位發展部的[數位皮夾](https://wallet.gov.tw/zh-tw)作為主要報到方式，此後臺僅在非數位皮夾到或需要確認報到狀況時由工作人員操作。其會同步數位皮夾報到與人工報到的資料（未實作），避免重複報到。目前處於 Demo 狀態，目前多數程式碼由 Codex 協助產生及修改。

It shows a spreadsheet-like attendee table with name, ticket ID, ticket type, registration checkbox, and live registration status.

The app can run in two modes:

- Frontend-only demo mode: uses built-in sample data in the browser. No database or backend is required.
- Full mode: uses an Axum backend, Postgres, SQLx migrations, and a WebSocket channel to sync registration status across browser sessions.

## Prerequisites

- Rust 1.85.0 with the `wasm32-unknown-unknown` target
- `trunk`
- PostgreSQL, only for full mode
- `sqlx-cli`, optional but useful for managing migrations manually

Example setup (`rustup` is recommended):

```sh
rustup toolchain install 1.85.0
rustup target add wasm32-unknown-unknown --toolchain 1.85.0
cargo install trunk
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### For Nix User Only

If you want to use the included flake file, copy `flake.nix.example` to `flake.nix`, and enter the dev shell instead:

```sh
nix develop
```

The flake shell provides stable Rust 1.85.0, rust-analyzer, Trunk, cargo-leptos, PostgreSQL client tools, and sqlx-cli.

## Configuration

Copy the example config before running the backend:

```sh
cp config.example.toml config.toml
```

`config.toml` has two sections:

```toml
[server]
host = "127.0.0.1"
port = 3000
dist_dir = "dist"

[database]
url = "postgres://conference_user:conference_password@localhost:5432/conference_registration"
max_connections = 5
```

- `server.host`: address the Axum backend binds to.
- `server.port`: backend HTTP/WebSocket port.
- `server.dist_dir`: directory containing the Trunk-built frontend.
- `database.url`: Postgres connection string.
- `database.max_connections`: SQLx pool size.

The backend uses `config.toml` automatically when it exists. You can override the path with:

```sh
APP_CONFIG=/path/to/config.toml cargo run --features server --bin server
```

## Mock Data And Migration

The database schema and seed data live in:

```sh
migrations/20260430000100_create_attendees.sql
```

The migration creates an `attendees` table:

The insert uses `ON CONFLICT (ticket_id) DO NOTHING`, so rerunning migrations
does not overwrite existing attendee rows.

## Run Without Postgres

Use this mode to see the UI without a backend or database on port 8080:

```sh
trunk serve --address 127.0.0.1 --port 8080
```

Open:

```sh
http://127.0.0.1:8080/
```

Expected result:

- The page loads with built-in sample attendees.
- The top-right status shows sample/demo state instead of `Live Data`.
- Checking rows changes the browser state only.
- Refreshing resets to sample data because no backend is running.

## Run With Postgres

Create a Postgres database and user that match `config.toml`, or update
`database.url` to match your local setup.

Build the frontend:

```sh
trunk build
```

Run the backend:

```sh
cargo run --features server --bin server
```

Open:

```sh
http://127.0.0.1:3000/
```

Expected result:

- The backend connects to Postgres.
- SQLx runs the migration automatically.
- The page loads attendee rows from the `attendees` table.
- The top-right status changes to `Live Data` once the initial database fetch succeeds and the WebSocket opens.
- Checking or unchecking a row updates Postgres.
- Other open browser sessions receive the registration update over WebSocket.

For development, you can also run Trunk and the backend separately:

```sh
trunk serve --address 127.0.0.1 --port 8080
cargo run --features server --bin server
```

Open the Trunk URL. The frontend detects that it is running on a dev port and sends API/WebSocket traffic to the backend on port `3000`.

## Notes

This is not production ready. Data syncing with verification server, authentication, authorization, logs, attendee search etc. are not yet implemented.

與數位皮夾驗證 API 溝通、權限控管及眾多 production 功能還未實作。
