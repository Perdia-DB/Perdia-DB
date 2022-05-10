FROM rust:latest

COPY ./ ./

EXPOSE 80
RUN cargo build --release

CMD ["./target/release/perdia_db.exe"]