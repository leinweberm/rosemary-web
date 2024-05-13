FROM alpine:latest
WORKDIR /app
COPY ./target/release/rest_api /app/
CMD ["./rest_api"]