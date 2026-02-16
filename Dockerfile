FROM rust:1.93 AS builder

# RUN apk update \
#     && apk add ca-certificates \
#     libssl3 \
#     openssl \
#     libssl-dev \
#     curl \
#     bash \
#     wget

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY Dioxus.toml ./Dioxus.toml
COPY tailwind.css ./tailwind.css

RUN mkdir src && echo 'fn main() {println!("Load Dependencies!");}' > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

COPY assets ./assets
COPY src ./src

# Create the final bundle folder. Bundle with release build profile to enable optimizations.
RUN dx bundle --web --release

# FROM alpine:latest
FROM debian:stable-slim

# Install runtime library dependencies
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# RUN apk update \
#     && apk add ca-certificates \
#     libssl3 \
#     curl \
#     wget \
#     && rm -rf /var/cache/apk/*

WORKDIR /app

COPY --from=builder /app/target/dx/proteus-website/release/web /app

ENV RUST_LOG=info
ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

CMD ["/app/cmccdb"]
