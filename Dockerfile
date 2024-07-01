ARG BASE_IMAGE=dolphinjiang/rust-musl-builder:1.78.0
FROM ${BASE_IMAGE} AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release

FROM alpine:3.18.2
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
COPY --from=builder /home/rust/src/settings.toml /app
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/infra-server /app
COPY --from=builder /home/rust/src/log4rs.yaml /app
ENV TZ=Asia/Shanghai
# RUN cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
RUN apk update && apk add curl websocat zlib zlib-dev openssl-dev openssl tzdata musl-locales
CMD ["./chat-hub"]