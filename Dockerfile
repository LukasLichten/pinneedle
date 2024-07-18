FROM rust:1.79.0-alpine3.20 AS builder

WORKDIR /build

COPY src ./src
COPY Cargo.* ./

RUN apk add musl-dev
RUN cargo build --release

FROM alpine:3.20

WORKDIR /app

RUN apk add --no-cache git
COPY --from=builder /build/target/release/pinneedle /usr/bin

EXPOSE 3000

ENTRYPOINT [ "pinneedle" ]
