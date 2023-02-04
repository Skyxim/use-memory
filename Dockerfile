FROM rust:alpine as builder
RUN apk add musl-dev
COPY . /src
WORKDIR /src
RUN cargo build --release
FROM alpine:latest
COPY --from=builder /src/target/release/memory /app
ENTRYPOINT [ "/app" ]