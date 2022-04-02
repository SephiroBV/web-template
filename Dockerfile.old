ARG APP_NAME=web_template
ARG RUST_VERSION=1.59.0
ARG CARGO_CHEF_VERSION="=0.1.35"
ARG USERNAME=runner

FROM clux/muslrust:${RUST_VERSION}-stable AS chef
USER root
ARG CARGO_CHEF_VERSION
RUN cargo install cargo-chef --version $CARGO_CHEF_VERSION
WORKDIR /app

FROM chef AS planner
COPY /Cargo.toml .
COPY /Cargo.lock .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG APP_NAME
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY /src src
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine AS runtime
ARG APP_NAME
ARG USERNAME
ENV APP_NAME $APP_NAME
RUN addgroup -S $USERNAME && adduser -S $USERNAME -G $USERNAME
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/$APP_NAME /usr/local/bin/
USER $USERNAME
CMD "/usr/local/bin/$APP_NAME"