FROM rust:latest as builder

RUN USER=root cargo new --bin fawkes-api
WORKDIR ./fawkes-api
COPY ./Cargo.toml ./Cargo.toml
ADD https://mirror.cs.uchicago.edu/fawkes/files/1.0/fawkes_binary_linux-v1.0.zip ./
RUN apt -y install musl-tools
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm src/*.rs

ADD . ./

RUN cargo build --target x86_64-unknown-linux-musl --release



FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
      && apt-get install -y ca-certificates tzdata \
      && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /fawkes-api/target/release/fawkes-api ${APP}/
COPY --from=builder /fawkes-api/protection ${APP}/
RUN mkdir ${APP}/uploads
