#!/usr/bin/env bash
set -euo pipefail

IMAGE="${IMAGE:-hytale-curseforge-headlessclient}"
CONTAINER_NAME="${CONTAINER_NAME:-hytale-curseforge-headlessclient}"

# Host paths (underscores)
BASE_DIR="${BASE_DIR:-/opt/hytale_curseforge_headlessclient}"
CONFIG_DIR="${CONFIG_DIR:-$BASE_DIR/config}"
LOG_DIR="${LOG_DIR:-$BASE_DIR/logs}"
CACHE_DIR="${CACHE_DIR:-$BASE_DIR/cache}"

# AMP instance root (override if needed)
INSTANCE_ROOT="${INSTANCE_ROOT:-/home/amp/.ampdata/instances/Hytale01/hytale}"

XAUTH_FILE="/tmp/docker.xauth"

die() { echo "ERROR: $*" >&2; exit 1; }

# User who owns the SSH/X11 session (must not be root)
INVOKER="${SUDO_USER:-$USER}"

[[ -n "${DISPLAY:-}" ]] || die "DISPLAY is empty. Reconnect in MobaXterm with X11 forwarding enabled."
command -v xauth >/dev/null 2>&1 || die "xauth not found. Install: sudo apt-get install -y xauth"
sudo -v || die "sudo auth failed"

# Auto-detect instance root if default isn't accessible/doesn't exist
if ! sudo test -d "$INSTANCE_ROOT" 2>/dev/null; then
  found="$(sudo sh -lc 'ls -d /home/amp/.ampdata/instances/*/hytale 2>/dev/null | head -n 1' || true)"
  [[ -n "$found" ]] || die "No AMP instance found at /home/amp/.ampdata/instances/*/hytale. Set INSTANCE_ROOT=... and retry."
  INSTANCE_ROOT="$found"
fi

RUN_UID="$(sudo stat -c '%u' "$INSTANCE_ROOT")"
RUN_GID="$(sudo stat -c '%g' "$INSTANCE_ROOT")"

# Persistent host state dirs (config/logs/cache)
sudo mkdir -p "$CONFIG_DIR" "$LOG_DIR" "$CACHE_DIR"
sudo chown -R "${RUN_UID}:${RUN_GID}" "$CONFIG_DIR" "$LOG_DIR" "$CACHE_DIR"
sudo chmod 775 "$CONFIG_DIR" "$LOG_DIR" "$CACHE_DIR"

# Build an Xauthority file readable by the container user
if sudo test -d "$XAUTH_FILE" 2>/dev/null; then
  sudo rm -rf "$XAUTH_FILE"
fi
sudo rm -f "$XAUTH_FILE"
sudo touch "$XAUTH_FILE"
sudo chmod 600 "$XAUTH_FILE"

if ! sudo -u "$INVOKER" xauth nlist "$DISPLAY" | grep -q .; then
  die "No X11 cookie found for DISPLAY=$DISPLAY as user $INVOKER. Reconnect in MobaXterm with X11 forwarding enabled."
fi

sudo -u "$INVOKER" xauth nlist "$DISPLAY" \
  | sed -e 's/^..../ffff/' \
  | sudo xauth -f "$XAUTH_FILE" nmerge - >/dev/null

sudo chown "${RUN_UID}:${RUN_GID}" "$XAUTH_FILE"
sudo chmod 600 "$XAUTH_FILE"

echo "Image:         $IMAGE"
echo "DISPLAY:       $DISPLAY"
echo "Instance root: $INSTANCE_ROOT"
echo "Run as:        uid=$RUN_UID gid=$RUN_GID"
echo "Config:        $CONFIG_DIR -> /data/config (XDG_CONFIG_HOME)"
echo "Cache:         $CACHE_DIR -> /data/cache  (XDG_CACHE_HOME)"
echo "Logs:          $LOG_DIR -> /data/logs"

# Ensure only one container instance is running
if docker ps -a --format '{{.Names}}' | grep -qx "$CONTAINER_NAME"; then
  docker rm -f "$CONTAINER_NAME" >/dev/null 2>&1 || true
fi

exec docker run --rm -it \
  --name "$CONTAINER_NAME" \
  --network=host \
  --user "${RUN_UID}:${RUN_GID}" \
  -e DISPLAY="$DISPLAY" \
  -e XAUTHORITY=/tmp/.Xauthority \
  -e HOME=/tmp \
  -e XDG_CONFIG_HOME=/data/config \
  -e XDG_CACHE_HOME=/data/cache \
  -v "${XAUTH_FILE}:/tmp/.Xauthority:ro" \
  -v "${CONFIG_DIR}:/data/config:rw" \
  -v "${CACHE_DIR}:/data/cache:rw" \
  -v "${LOG_DIR}:/data/logs:rw" \
  -v "${INSTANCE_ROOT}:/hytale:rw" \
  "$IMAGE"
