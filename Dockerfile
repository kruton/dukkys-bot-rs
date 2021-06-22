# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM ekidd/rust-musl-builder AS build

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

RUN sudo chown -R rust:rust /home

# Install and cache dependencies layers
# Rather than copying everything every time, re-use cached dependency layers
# to install/build deps only when Cargo.* files change.
RUN USER=root cargo new /home/dukkys-bot-rs --bin

WORKDIR /home/dukkys-bot-rs

# Download the dependencies so we don't have to do this every time.
COPY Cargo.toml Cargo.lock ./
RUN echo "fn main() {}" > dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
RUN rm dummy.rs

# Copy the source and build the application.
COPY src ./src

RUN cargo build --bins --release --target x86_64-unknown-linux-musl
RUN find /home/dukkys-bot-rs/ -name dukkys-bot-rs

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /home/dukkys-bot-rs/target/x86_64-unknown-linux-musl/release/dukkys-bot-rs .
USER 1000
ENTRYPOINT ["./dukkys-bot-rs"]
