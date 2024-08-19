# use rust runtime
FROM rust:1.80.1 AS build

#port source to docker
COPY . /yt_web

#compile
WORKDIR /yt_web
RUN cargo build --release

#Second step
FROM debian:bullseye-slim

# port binary
COPY --from=build /yt_web/target/release/yt_web /main/yt_web
COPY --from=build /yt_web/static /main/static
WORKDIR /main

CMD [ "yt_web" ]

