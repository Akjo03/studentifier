FROM rust:1.66.0

WORKDIR /usr/src/server
COPY . .

RUN cargo build --release

EXPOSE 3000

CMD [ "target/release/studentifier-server" ]