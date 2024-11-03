ARG RUST_VERSION=1.82-alpine
ARG ALPINE_VERSION=3.20

FROM rust:$RUST_VERSION as builder
WORKDIR /app
COPY . .
RUN rustup install nightly
RUN apk add --no-cache musl-dev libgcc openssl-dev
RUN cargo +nightly install --path .

FROM alpine:$ALPINE_VERSION
RUN apk add --no-cache libstdc++ openssl
COPY --from=builder /usr/local/cargo/bin/granja_bot /usr/local/bin/granja_bot
EXPOSE 3600
CMD ["/usr/local/bin/granja_bot"]
