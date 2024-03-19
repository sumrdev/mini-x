FROM alpine:edge as BUILDER

WORKDIR /usr/src/mini-x

RUN apk update
RUN apk add --no-cache rust
RUN apk add --no-cache cargo
RUN apk add --no-cache libpq-dev

# Copy over the Cargo.toml files to the shell project
COPY Cargo.toml Cargo.lock ./

# Build and cache the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

COPY . .

RUN cargo build --release 

FROM alpine:edge

WORKDIR /usr/src/mini-x

COPY --from=BUILDER /usr/src/mini-x/target/release ./

COPY src/frontend/static/ .
RUN apk update
RUN apk add libc6-compat
RUN apk add libgcc
RUN apk add libpq

EXPOSE 5000
EXPOSE 5001

CMD ["./mini-x"]