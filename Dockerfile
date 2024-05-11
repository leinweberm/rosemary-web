# Start from a Debian base image
FROM debian:buster-slim

# Set the working directory in the Docker image
WORKDIR /app

# Copy the prebuilt binary file into the Docker image
COPY ./target/release/rest_api /app/

# Set the binary as the command to run when the container starts
CMD ["./rest_api"]