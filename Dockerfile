FROM rust:1.35.0-slim

RUN apt-get update
RUN apt-get -y install locales-all
RUN apt-get -y install tmux

COPY . ~/trust
WORKDIR ~/trust

RUN cargo build

ENTRYPOINT ["cargo", "test"]
