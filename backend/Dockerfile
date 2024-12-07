FROM rust:1.75 as builder
WORKDIR /usr/src/app
# まずマニフェストファイルをコピーして依存関係をキャッシュ
COPY Cargo.toml Cargo.lock ./
# ダミーのソースをコピーして依存関係をビルド
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release
# 実際のソースコードをコピー
COPY src ./src
# 再度ビルド
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/src/app
# SSL証明書をインストール
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /usr/src/app/target/release/backend ./
COPY .env ./.env
COPY private-key.pem ./private-key.pem
RUN chmod +x ./backend
ENV RUST_LOG=debug
EXPOSE 8080
CMD ["./backend"]