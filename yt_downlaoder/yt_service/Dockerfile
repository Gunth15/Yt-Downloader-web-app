# use rust runtime
FROM rust:1.80.1 AS build

#port source to docker
COPY . /yt_service
# goto crate dir
WORKDIR /yt_service

#compile
RUN cargo build --release

#Second step
FROM debian:bookworm-slim

#update and install libssl
RUN apt-get update && apt-get -y upgrade
RUN apt-get -y install libssl3

# port binary
COPY --from=build /yt_service/target/release/yt_service /api/yt_service
WORKDIR /api

CMD [ "./yt_service" ]

