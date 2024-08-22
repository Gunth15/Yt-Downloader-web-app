# use rust runtime
FROM rust:1.80.1 AS build

#port source to docker
COPY . /yt_web

#change dir to crate
WORKDIR /yt_web
#compile
RUN cargo build --release

#Second step
FROM debian:bookworm-slim

#update and install libssl
RUN apt-get update && apt-get -y upgrade
RUN apt-get -y install libssl3

# port binary
COPY --from=build /yt_web/target/release/yt_web /main/yt_web
COPY --from=build /yt_web/static /main/static
WORKDIR /main

CMD [ "./yt_web" ]

