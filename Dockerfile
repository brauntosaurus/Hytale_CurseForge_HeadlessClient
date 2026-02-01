# syntax=docker/dockerfile:1

# --- build stage ---
FROM rust:1-bookworm AS build
WORKDIR /app

# Build deps for Dioxus Desktop (wry/webkit/gtk) + TLS
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    clang \
    cmake \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
  && rm -rf /var/lib/apt/lists/*

# (Optional) Cargo dep-cache layer: build a dummy main first
COPY Cargo.toml Cargo.lock build.rs ./
RUN mkdir -p src && printf '%s\n' 'fn main(){}' > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Now copy the full source and build for real
COPY . .
RUN cargo build --release

# --- runtime stage ---
FROM debian:bookworm-slim
WORKDIR /app

# Runtime libs for Dioxus Desktop (WebKitGTK/GTK) + X11 + basic GL + TLS
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libglib2.0-0 \
    libcairo2 \
    libpango-1.0-0 libpangocairo-1.0-0 \
    libgdk-pixbuf-2.0-0 \
    libatk1.0-0 libatk-bridge2.0-0 \
    libgtk-3-0 \
    libwebkit2gtk-4.1-0 \
    libx11-6 libx11-xcb1 libxcb1 libxkbcommon0 \
    libdrm2 libgbm1 \
  && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary (crate name from Cargo.toml)
COPY --from=build /app/target/release/HytaleModManager /usr/local/bin/HytaleModManager

CMD ["HytaleModManager"]
