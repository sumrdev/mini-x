FROM alpine:edge as BUILDER

WORKDIR /usr/src/mini-x

COPY . .
RUN apk update
RUN apk add --no-cache sqlite-dev
RUN apk add --no-cache rust
RUN apk add --no-cache cargo

RUN cargo build --release 

FROM alpine:edge

WORKDIR /usr/src/mini-x

COPY --from=BUILDER /usr/src/mini-x/target/release ./

COPY src/frontend/static/ .
RUN apk update
RUN apk add libc6-compat
RUN apk add sqlite-libs
RUN apk add libgcc

EXPOSE 5000
EXPOSE 5001

CMD ["./mini-x"]