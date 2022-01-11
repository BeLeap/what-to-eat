FROM rust:latest

WORKDIR /usr/app/what-to-eat
COPY . .

RUN cargo install --path .
CMD ["what-to-eat"]
