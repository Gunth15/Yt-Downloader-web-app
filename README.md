# Yt-Downloader-web-app

## About this project

This project was build as an excuse to learn rust as a language first and foremost.
I've ended learning way more about web development then I though I would this summer.
This is a simple youtube downloader web server that is supposed to be ran on a machine to take request from clients and downloads youtube videos to the server.

### Built With

- ![Actix web](https://actix.rs/img/logo.png) [https://actix.rs/]
- ![postgresql](https://www.postgresql.org/media/img/about/press/elephant.png) [https://www.postgresql.org/]
- ![Python3.11][https://www.python.org/static/img/python-logo.png](https://www.python.org/) Python 3.11
- ![Rust][https://www.rust-lang.org/static/images/rust-logo-blk.svg](https://www.rust-lang.org/)
- [Pytubefix](https://github.com/JuanBindez/pytubefix)
- [Serde_json](https://github.com/serde-rs/json)
- [Tera](https://github.com/Keats/tera)

#### optional deployment method(WIP)

![DOCKER](https://driftt.imgix.net/https%3A%2F%2Fdriftt.imgix.net%2Fhttps%253A%252F%252Fs3.us-east-1.amazonaws.com%252Fcustomer-api-avatars-prod%252F5244849%252Fb3353cad7116db6f9be2bc43cfbc048374xfdtnudd3c%3Ffit%3Dmax%26fm%3Dpng%26h%3D200%26w%3D200%26s%3D7ff0c42ec0bafa67064f35811896732f?fit=max&fm=png&h=200&w=200&s=e33d56d911e571a3b45193bc603e41a1) [https://www.docker.com/]

## Getting Started

### Prerequisites

Install these dependencies according to your operating system

- Rust
- Python 3.11
- Postgresql

### Installation

Clone the github repo
`git clone https://github.com/Gunth15/Yt-Downloader-web-app`

#### Postgres setup
>
> [!NOTE]
> May want to change defaults for db if hosting this service for whatever reason

Create two databases one for the video api and the users login credentials in postgres
`Create databse youtube_service`
`Create databse dl_users`
Create credentials  for each database
`Create <User> with password <password>`
`Create <User> with password <password>`

Run scripts in each database to create necessary tables
> [!NOTE]
> If user was changed form the default, adjust script for changes

`psql -d dbscript.sql -d <database_name>`
where database_name is db_user for yt_web and youtube_service for yt_service unless changed from defaults.

#### .env files

Each directory has their own .env file where you can set the port and ip of the sever and clients they use. Aswell as set the db

Database host address and other addresses should be adjusted for any changes made from defaults

Here's a rundown of each .env file

1. Main directory .env, purely used for docker(WIP)

2. Yt_downloader_manager, only has a option to choose host address

3. yt_service, CLIENT_PORT=Yt_downloader_manager,

4. yt_web, CLIENT_PORT=yt_service

Everything else about the .env files should be self explanatory

> [!NOTE]
> Can also set environment variables by declaring them in your terminal session like you would in a normal bash session

#### Python script dependency

create a python3.11 virtual environment
`python3.11 -m venv .penv && source .penv`
install pytubefix
`pip install pytubefix`

If you compile as a binary, make sure to `source .penv` before starting Yt_downloader_manager

#### Running/building the rust binary

For a quick showcase, you can `cargo run` in each crate within the project directory except pytube_wrpr and interact with whole stack using yt_web host addr

If you plan to build the project using `cargo build --release`, make sure to include a `/downloads` folder with your `yt_downloader_manager` binary location
and all of `/static` with the `yt_web` binary

DON'T FORGET TO SOURCE .PENV BEFORE USING `yt_downloader_manager`

## Docker

Compiling with docker is a WIP
