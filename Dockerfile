FROM rust:1.97.1-alpine3.22 AS builder
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/webserver
COPY . .
RUN cargo build --release

FROM alpine:3.22
COPY --from=builder /usr/src/webserver/target/release/webserver /
CMD ["./webserver"]

