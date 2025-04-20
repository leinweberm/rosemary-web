# FROM rust:alpine AS build
# RUN apk add --no-cache musl-dev build-base pkgconfig
# ENV RING_ALLOW_MISSING_ASM=1
# RUN rustup target add x86_64-unknown-linux-musl
# WORKDIR /app
# RUN USER=root
# RUN apk add --no-cache musl-dev pkgconfig build-base
# RUN --mount=type=bind,source=src,target=src \
# 	--mount=type=bind,source=templates,target=templates \
# 	--mount=type=bind,source=Cargo.toml,target=Cargo.toml \
# 	--mount=type=bind,source=Cargo.lock,target=Cargo.lock \
# 	--mount=type=cache,target=/app/target/ \
# 	--mount=type=cache,target=/usr/local/cargo/registry/ \
# 	<<EOF
# set -e
# cargo build --locked --release --target x86_64-unknown-linux-musl
# cp ./target/x86_64-unknown-linux-musl/release/rosemary /bin/server
# EOF
#
FROM scratch AS final
COPY ./target/x86_64-unknown-linux-musl/release/rosemary /app/rosemary
WORKDIR /app
EXPOSE 3030
ENV RUST_LOG=debug
CMD ["/app/rosemary"]