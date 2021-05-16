FROM rust:1.52.1 AS build

WORKDIR /usr/src/rustycheckers

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release --target wasm32-unknown-unknown
RUN cp target/wasm32-unknown-unknown/release/rustycheckers.wasm demo/
RUN pwd
RUN ls demo/

FROM ubuntu:latest as web
USER root

RUN apt-get update
RUN apt-get install -y nginx nodejs
RUN rm -v /etc/nginx/nginx.conf
COPY --from=build /usr/src/rustycheckers/demo /usr/share/nginx/html/
RUN echo "daemon off; worker_processes 1; events { worker_connections 1024; } http { include    mime.types;    sendfile on;    server {       root /usr/share/nginx/html/;        index index.html;        server_name localhost;        listen 80;    }}" >> /etc/nginx/nginx.conf
EXPOSE 80
CMD service nginx start