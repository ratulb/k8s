FROM rust:alpine  as builder
WORKDIR /usr/rocket-app_v1
RUN apk add --no-cache musl-dev
COPY ./ ./
RUN cargo build --release

FROM alpine

COPY --from=builder target/release/rocket-app_v1 /usr/local/bin/rocket-app_v1

CMD ["rocket-app_v1"]

