FROM rust:1.74.1
WORKDIR /usr/src/app
COPY ./app .
RUN ["rustc", "./main.rs"]
CMD ./main
