FROM rust:latest

WORKDIR /usr/src/rust-todo-app

COPY . .

RUN cargo install --path .

CMD ["rust-todo-app"]
