#run in parent directory
#start rust build time and add pip and python
FROM rust:1.80.1 AS build

#copy pytubewrpr dependency
COPY /pytube_wrpr/ /pytube_wrpr

#copy crate to image and set as dir 
COPY /yt_download_manager/ /download_manager
WORKDIR /download_manager


#compile python dependency
RUN apt-get update && apt-get install -y python3 python3-pip
# Compile rust biary
RUN cargo build --release


#second build 
FROM python:3.11
#copy rust build
COPY --from=build /download_manager/target/release/yt_download_manager /download_manager/yt_download_manager
WORKDIR /download_manager
#source python virtual enviornment and install pytubefix
RUN python3.11 -m venv venv && . venv/bin/activate
RUN pip install --no-cache-dir pytubefix

#make downloads diretory
RUN mkdir /downloads

#start app
CMD [ "./yt_download_manager" ]


