FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/target/release/shorty /app/
EXPOSE 8080
CMD ["/app/shorty"]