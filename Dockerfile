# Still doesn't work, axum project still crashes when accessed either from container or on host browser
FROM rust:alpine3.21 AS build

RUN apk update
RUN apk upgrade
RUN apk add --no-cache pkgconf openssl-dev musl-dev gcc curl
ENV OPENSSL_DIR=/usr

WORKDIR /app
COPY src /app/src
COPY Cargo.toml /app

RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release && \ 
    cp ./target/release/airoapi /bin/server 

FROM alpine:3.18 AS final

RUN apk update
RUN apk upgrade
RUN apk add --no-cache gcc curl

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

EXPOSE 8080

CMD ["/bin/server"] 