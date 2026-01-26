#!/usr/bin/env bash
set -euo pipefail

# --- Names / paths ---
IMAGE="hytale-curseforge-headlessclient"
RUNTIME_DIR="/opt/hytale_curseforge_headlessclient"

# Repo root = directory containing this deploy.sh
SRC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

RUNTIME_FILES=(
  "run.sh"
)

echo "[0/3] Preflight"
[[ -d "$SRC_DIR" ]] || { echo "Missing repo dir: $SRC_DIR" >&2; exit 1; }
[[ -f "$SRC_DIR/Dockerfile" ]] || { echo "Missing Dockerfile in repo root: $SRC_DIR" >&2; exit 1; }

echo "[1/3] Ensuring runtime dirs exist: $RUNTIME_DIR"
sudo mkdir -p "$RUNTIME_DIR"/{config,logs}
sudo touch "$RUNTIME_DIR/config/settings.json"

echo "[2/3] Installing runtime scripts into $RUNTIME_DIR"
for f in "${RUNTIME_FILES[@]}"; do
  if [[ -f "$SRC_DIR/$f" ]]; then
    sudo install -m 755 "$SRC_DIR/$f" "$RUNTIME_DIR/$f"
  else
    echo "WARN: $SRC_DIR/$f not found (skipping)"
  fi
done

echo "[3/3] Building docker image: $IMAGE"
docker build -t "$IMAGE" "$SRC_DIR"

echo "Done."
echo "Run: $RUNTIME_DIR/run.sh"
