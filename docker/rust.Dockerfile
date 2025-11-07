# dev.Dockerfile
FROM rust:1-slim-bookworm

WORKDIR /app

# ホットリロード用ツール
RUN cargo install cargo-watch

# OpenCVのインストール
RUN apt-get update \
  && apt-get install -y --no-install-recommends libopencv-dev \
  && rm -rf /var/lib/apt/lists/*

# ホスト側のソースをマウント
# VOLUME ["/apps/rust-server"]

CMD ["cargo", "watch", "-x", "run"]
