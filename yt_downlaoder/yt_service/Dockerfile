# use rust runtime
FROM rust:1.80.1 AS build

#port source to docker
COPY . /yt_service

#compile
WORKDIR /yt_service
RUN cargo build --release

#Second step
FROM debian:bullseye-slim

# port binary
COPY --from=build /yt_service/target/release/yt_service /api/yt_service
WORKDIR /api

CMD [ "yt_service" ]

