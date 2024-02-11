FROM rust:1.76

WORKDIR /usr/src/mini-x
COPY . .

EXPOSE 5000

RUN cargo install --path .

CMD ["mini-x"]