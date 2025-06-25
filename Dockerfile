FROM rust:1.87.0 as builder

WORKDIR /usr/src/portfolio
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/portfolio/target/release/portfolio ./portfolio
COPY --from=builder /usr/src/portfolio/static ./static
COPY --from=builder /usr/src/portfolio/templates ./templates
COPY --from=builder /usr/src/portfolio/data ./data

RUN chmod +x ./portfolio

ENV RUST_LOG=info
ENV ACTIX_ENV=production

EXPOSE 8080

CMD ["./portfolio"]
