FROM rust:1.74.1
WORKDIR /usr/src/app
COPY ./src ./src
COPY ./Cargo.toml .
RUN ["cargo", "run"]
CMD ["./target/debug/Task-7"]
