ARG BASE_IMAGE=rust:1.72-bullseye
FROM ${BASE_IMAGE} AS builder
WORKDIR /app
COPY . /app
RUN rustup default stable
RUN cargo build --release

FROM debian:bullseye-slim
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
COPY --from=builder /app/settings.toml /app
COPY --from=builder /app/target/release/chat-hub /app/
COPY --from=builder /app/log4rs.yaml /app
RUN apt-get update -y && apt-get install curl libpq5 -y
CMD ["./chat-hub"]