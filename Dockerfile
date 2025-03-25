FROM rust:1.83.0

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y curl libssl-dev libpq-dev postgresql-client openssl && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

COPY ./scripts/start.sh /app/start.sh
RUN chmod +x /app/start.sh

COPY ./scripts/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 21001 9000

ENTRYPOINT ["/app/entrypoint.sh"]
