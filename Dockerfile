FROM rust:latest as builder
WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock ./
COPY ./templates ./
RUN mkdir src
COPY src ./src
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache libstdc++
WORKDIR /usr/src/myapp
VOLUME ["/app/static"]
COPY --from=builder /usr/src/myapp/target/release/rosemary .
COPY ./src/certs/root.crt .
COPY .env .
EXPOSE 3030
ENV RUST_LOG=api=error,database=error,app=error,cfg=error
CMD ["./rosemary"]