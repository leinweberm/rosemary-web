# ROSEMARY WEB

- [About](#about)
- [Development](#development)
- [Deployment](#deployment)

## About
www.rosemary-artist.com are commercial websites including selling paintings, photography services and blog. Application is made of:
 - backend server written in rust providing CRUD rest API + serving static HTML fles for main website. 
 - admin SPA written in Vue 3

## Development
### requirements:
- docker installed
- docker compose 2.22+

## Deployment

### Files on VM

**certs:**
- /etc/letsencrypt/live/rosemary-artist.com/fullchain.pem;
- /etc/letsencrypt/live/rosemary-artist.com/privkey.pem;
**app files:**
- /var/www/rosemary-artist/

### Tokio tests
```cargo test -- --nocapture```