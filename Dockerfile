# compile project
FROM rust:latest as builder
WORKDIR /build
COPY . .
RUN cargo build --release

# build slim production image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /build/target/release/rosemary .
RUN chmod +x rosemary
EXPOSE 3030
CMD ["./rosemary"]
