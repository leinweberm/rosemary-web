# Nginx for local development

## check volume paths
### volumes
```
/home/leinweberm/Projects/personal/rosemary-web/static:/usr/share/nginx/html/static:ro

/home/leinweberm/Projects/personal/rosemary-web/development/nginx:/var/log/nginx

/home/leinweberm/Projects/personal/rosemary-web/development/nginx:/etc/nginx/nginx.conf:ro
```

## run
```shell
docker run -d \
  --name rosemary-proxy \
  -p 80:80 \
  -v /home/leinweberm/Projects/personal/rosemary-web/static:/usr/share/nginx/html/static:ro \
  -v /home/leinweberm/Projects/personal/rosemary-web/development/nginx:/var/log/nginx \
  -v /home/leinweberm/Projects/personal/rosemary-web/development/nginx/nginx.conf:/etc/nginx/nginx.conf:ro \
  nginx:latest

```