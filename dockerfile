FROM rust:buster as builder

RUN USER=root cargo new --bin fawkes-api
WORKDIR ./fawkes-api
COPY ./Cargo.toml ./Cargo.toml
ADD https://mirror.cs.uchicago.edu/fawkes/files/1.0/fawkes_binary_linux-v1.0.zip ./fawkes.zip
RUN unzip ./fawkes.zip
#RUN rustup target add x86_64-unknown-linux-musl
#--target x86_64-unknown-linux-musl
RUN cargo build  --release
RUN rm src/*.rs

ADD . ./

RUN cargo build --release



FROM python:slim-buster as app
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

RUN apt-get update
RUN apt-get install libglib2.0-0 libsm6 libxext6 libxrender-dev  libfontconfig1 -y
RUN pip install opencv-python

RUN mkdir ${APP}/uploads
