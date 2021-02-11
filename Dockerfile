FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release
EXPOSE 8080

RUN wget -O ./wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh \
    && chmod +x ./wait-for-it.sh
