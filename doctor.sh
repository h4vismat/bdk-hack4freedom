#!/usr/bin/env bash
#
# doctor.sh — checks that your machine is ready for the BDK workshop.
#
# Usage: ./doctor.sh

set -u

# Minimum Rust version: this project uses edition 2024 (Cargo.toml),
# which requires rustc 1.85 or newer.
MIN_RUST_MAJOR=1
MIN_RUST_MINOR=85

FAILURES=0

# Colors (disabled when not a terminal)
if [ -t 1 ]; then
  GREEN=$'\033[0;32m'
  RED=$'\033[0;31m'
  YELLOW=$'\033[0;33m'
  BOLD=$'\033[1m'
  RESET=$'\033[0m'
else
  GREEN='' RED='' YELLOW='' BOLD='' RESET=''
fi

ok()   { printf "  %s✓%s %s\n" "$GREEN" "$RESET" "$1"; }
fail() { printf "  %s✗%s %s\n" "$RED" "$RESET" "$1"; FAILURES=$((FAILURES + 1)); }
hint() { printf "    %s→ %s%s\n" "$YELLOW" "$1" "$RESET"; }

printf "%sBDK workshop doctor%s\n\n" "$BOLD" "$RESET"

# --- Rust ---------------------------------------------------------------
printf "%sRust%s\n" "$BOLD" "$RESET"

if command -v rustc >/dev/null 2>&1; then
  RUST_VERSION=$(rustc --version | awk '{print $2}')
  RUST_MAJOR=$(printf '%s' "$RUST_VERSION" | cut -d. -f1)
  RUST_MINOR=$(printf '%s' "$RUST_VERSION" | cut -d. -f2)

  if [ "$RUST_MAJOR" -gt "$MIN_RUST_MAJOR" ] || {
       [ "$RUST_MAJOR" -eq "$MIN_RUST_MAJOR" ] && [ "$RUST_MINOR" -ge "$MIN_RUST_MINOR" ]
     }; then
    ok "rustc $RUST_VERSION (>= $MIN_RUST_MAJOR.$MIN_RUST_MINOR required for edition 2024)"
  else
    fail "rustc $RUST_VERSION is too old — this project needs >= $MIN_RUST_MAJOR.$MIN_RUST_MINOR (edition 2024)"
    hint "Update with: rustup update stable"
  fi
else
  fail "rustc not found"
  hint "Install Rust: https://rustup.rs  (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)"
fi

if command -v cargo >/dev/null 2>&1; then
  ok "cargo $(cargo --version | awk '{print $2}')"
else
  fail "cargo not found"
  hint "cargo ships with rustup — install Rust via https://rustup.rs"
fi

# --- Docker -------------------------------------------------------------
printf "\n%sDocker%s\n" "$BOLD" "$RESET"

if command -v docker >/dev/null 2>&1; then
  ok "docker $(docker --version | sed 's/^Docker version //; s/,.*//')"

  if docker info >/dev/null 2>&1; then
    ok "Docker daemon is running"
  else
    fail "Docker is installed but the daemon is not running"
    hint "Start Docker Desktop (macOS/Windows) or run: sudo systemctl start docker (Linux)"
  fi
else
  fail "docker not found"
  hint "Install Docker Desktop: https://docs.docker.com/get-docker/"
fi

# --- Nigiri -------------------------------------------------------------
printf "\n%sNigiri%s\n" "$BOLD" "$RESET"

if command -v nigiri >/dev/null 2>&1; then
  NIGIRI_VERSION=$(nigiri version 2>/dev/null | awk '/^Version:/ {print $2}')
  ok "nigiri ${NIGIRI_VERSION:-(version unknown)}"
else
  fail "nigiri not found"
  hint "Install Nigiri: curl https://getnigiri.vulpem.com | bash"
fi

# --- Summary ------------------------------------------------------------
printf "\n"
if [ "$FAILURES" -eq 0 ]; then
  printf "%s%sAll checks passed — you're ready for the workshop! 🎉%s\n" "$GREEN" "$BOLD" "$RESET"
  exit 0
else
  printf "%s%s%d check(s) failed.%s Fix the issues above and run ./doctor.sh again.\n" "$RED" "$BOLD" "$FAILURES" "$RESET"
  exit 1
fi
