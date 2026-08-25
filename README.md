# webserver

A minimal Rust web server built with [Axum](https://github.com/tokio-rs/axum) and Tokio.

## What it does

- Starts an HTTP server on `0.0.0.0:3000`
- Responds to `GET /` with `200 OK`

## Requirements

- Rust toolchain with Cargo

## Run

```bash
cargo run
```

Then visit:

```bash
curl http://localhost:3000/
```

You should get an empty `200 OK` response.

## Project structure

```text
src/main.rs
```

