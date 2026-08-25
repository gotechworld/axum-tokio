# webserver

A minimal Rust web server built with [Axum](https://github.com/tokio-rs/axum) and Tokio.

## What it does

- Starts an HTTP server on `0.0.0.0:3000`
- Responds to `GET /` with `200 OK`

## Key technologies

- **Rust** for the application code
- **Axum** for routing and HTTP server integration
- **Tokio** for the async runtime and TCP listener
- **Docker** for containerized builds and runtime packaging
- **GitHub Actions** for CI checks such as formatting, linting, build, tests, dependency review, and CodeQL scanning

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
Cargo.toml
README.md
Dockerfile
src/
  main.rs
.github/
  workflows/
    ci.yml
```

## Code organization

The application is intentionally small and currently lives in a single source file:

- `/home/runner/work/axum-tokio/axum-tokio/src/main.rs` is the entrypoint and contains the whole app
- `index_handler()` handles `GET /` and returns `200 OK`
- `main()` creates the Axum router, binds a Tokio `TcpListener` to `0.0.0.0:3000`, and starts serving requests

## Request flow

1. The Tokio runtime starts from `main()`
2. Axum builds a router with the `/` route
3. Incoming `GET /` requests are dispatched to `index_handler()`
4. The handler returns an empty `200 OK` response

## Repository layout details

- `/home/runner/work/axum-tokio/axum-tokio/Cargo.toml` defines the package metadata and dependencies
- `/home/runner/work/axum-tokio/axum-tokio/src/main.rs` contains the server code
- `/home/runner/work/axum-tokio/axum-tokio/Dockerfile` builds a release binary in a Rust image and copies it into a small Alpine runtime image
- `/home/runner/work/axum-tokio/axum-tokio/.github/workflows/ci.yml` defines the CI pipeline used by GitHub Actions

## Current architecture

This repository is a starter-style service with:

- one binary crate
- one route
- no separate modules yet
- no database, authentication, middleware, or shared application state
- no `tests/` or `examples/` directories at the moment
