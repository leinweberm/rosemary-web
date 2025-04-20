# Rosemary Web

## run with debug
```
RUST_LOG=app=debug,db=debug,api=debug,cfg=debug cargo run
```

## file structure DEV:
```
src
├- certs
|  ├ root.crt
|  └ root.key
└- .env
```

## Run in docker:
1. build binary locally with musl target:
  - `cargo build --locked --release --target x86_64-unknown-linux-musl`
2. build docker image:
  - `docker build --no-cache -t rosemary .`
3. run docker image in container:
```
docker run --rm \
  -p 3030:3030 \
  -v /home/leinweberm/Projects/personal/rosemary-web/.env.dev:/app/.env \
  -v /home/leinweberm/Projects/personal/rosemary-web/src/certs:/app/src/certs \
  -v /home/leinweberm/Projects/personal/rosemary-web/static/www:/app/www/static \
  rosemary
```