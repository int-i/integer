FROM rust:alpine AS build

WORKDIR /usr/src/integer

RUN apk add --no-cache build-base openssl-dev postgresql-dev

COPY . .

RUN cargo build --release

FROM alpine:3.14

WORKDIR /usr/local/bin/integer

RUN apk add --no-cache tzdata
ENV TZ Asia/Seoul

COPY --from=build /usr/src/integer/target/release/integer .

CMD ["./integer"]

