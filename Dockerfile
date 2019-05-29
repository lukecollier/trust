FROM rust:1.35.0-slim

RUN apt-get update
RUN apt-get -y install tmux
COPY . ~/trust
WORKDIR ~/trust

ENTRYPOINT ["cargo", "test"]
