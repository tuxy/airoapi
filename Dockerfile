FROM rust:alpine3.21

COPY ./ ./

RUN apk update
RUN apk upgrade
RUN apk add --no-cache pkgconf openssl-dev musl-dev cmake make gcc g++ nodejs perl clang16 curl strace
ENV OPENSSL_DIR=/usr
RUN cargo build --release

CMD ["./target/release/airoapi"]