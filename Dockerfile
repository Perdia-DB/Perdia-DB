FROM rust:latest

COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
COPY ./.env /app/.env
WORKDIR /app

EXPOSE 3000

RUN cargo build --release

CMD /app/target/release/perdia_db