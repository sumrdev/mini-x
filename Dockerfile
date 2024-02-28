FROM rust:1.76

WORKDIR /usr/src/mini-x
COPY . .

EXPOSE 5000

RUN cargo build -r

RUN mv templates/ target/release/
RUN mv static/ target/release/
RUN mv schema.sql target/release/

WORKDIR /usr/src/mini-x/target/release

CMD ["./mini-x"]