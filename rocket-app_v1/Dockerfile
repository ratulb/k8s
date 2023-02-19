FROM rust:alpine  as builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

FROM alpine

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rocket-app_v1 .
COPY --from=builder /app/Rocket.toml .

EXPOSE 8080

CMD ["./rocket-app_v1"]

