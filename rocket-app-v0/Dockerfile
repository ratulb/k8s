FROM rust:alpine  as builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY ./src ./src
COPY ./Cargo.toml .
COPY ./Rocket.toml .
RUN cargo build --release

FROM alpine

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rocket-app-v0 .
COPY --from=builder /app/Rocket.toml .

EXPOSE 8080

CMD ["./rocket-app-v0"]

