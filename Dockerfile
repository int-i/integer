# syntax=docker/dockerfile:1
FROM rust:alpine AS builder

WORKDIR /usr/src/integer

RUN apk add --no-cache g++ postgresql-dev

RUN cargo init .
COPY Cargo* ./
RUN cargo build --release \
    && rm target/release/deps/integer*

COPY . .

RUN cargo build --release

FROM alpine:3.14

WORKDIR /usr/local/bin/integer

RUN apk add --no-cache tzdata
ENV TZ Asia/Seoul

COPY --from=builder /usr/src/integer/target/release/integer .

EXPOSE 3000
CMD ["./integer"]

