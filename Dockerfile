FROM rust:latest

COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
WORKDIR /app

EXPOSE 3000/tcp

RUN cargo build --release

CMD /app/target/release/perdia_db