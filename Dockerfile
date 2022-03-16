FROM rust:latest

WORKDIR /usr/src/rust-todo-app

COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["rust-todo-app"]
