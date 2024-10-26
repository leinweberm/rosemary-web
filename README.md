# ROSEMARY WEB

## Server
### Files on VM
**certs:**
- /etc/letsencrypt/live/rosemary-artist.com/fullchain.pem;
- /etc/letsencrypt/live/rosemary-artist.com/privkey.pem;
**app files:**
- /var/www/rosemary-artist/

### Build
```docker build -t rosemary-server:prod -f Dockerfile .```

### Run
```docker run -v /var/www/rosemary-artist/static:app/static -p 3030:3030 rosemary-server```

### Run docker-compose
```docker-compose up --build```

### Tokio tests
```cargo test -- --nocapture```