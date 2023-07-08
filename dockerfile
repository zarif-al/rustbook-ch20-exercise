FROM rust:1.70

WORKDIR /usr/src/rustbook-ch20

COPY . .

RUN cargo build --release

EXPOSE 7878

CMD ["./target/release/rustbook-ch20-exercise"]




