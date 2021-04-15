# actix-image-upload
** WARNING: UNBOUNDED DISK USAGE. There are no safeguards to limit disk usage of this server **

A simple actix-web image upload server that uses a streamed body with the possiblity to handle additional url-encoded query parameters. Data is written into the disk using web::block (a threadpool).

Environment variable examples:
```
IMAGE_FOLDER=./images
BIND_ADDRESS=127.0.0.1:8080
```
