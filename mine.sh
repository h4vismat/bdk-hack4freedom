#!/usr/bin/env bash
#
# mine.sh — mines blocks on Nigiri's regtest chain.
#
# Usage: ./mine.sh <blocks>

set -euo pipefail

if [ $# -ne 1 ] || ! [[ "$1" =~ ^[1-9][0-9]*$ ]]; then
  echo "Usage: ./mine.sh <blocks>   (blocks must be a positive integer)" >&2
  exit 1
fi
BLOCKS=$1

if ! command -v nigiri >/dev/null 2>&1; then
  echo "Error: nigiri not found. Run ./doctor.sh to check your setup." >&2
  exit 1
fi

# Fails if the Nigiri stack isn't up
if ! nigiri rpc getblockcount >/dev/null 2>&1; then
  echo "Error: Nigiri doesn't seem to be running. Start it with: nigiri start" >&2
  exit 1
fi

START_HEIGHT=$(nigiri rpc getblockcount)

ADDRESS=$(nigiri rpc getnewaddress "" "bech32")
echo "Mining $BLOCKS block(s) to $ADDRESS ..."

for i in $(seq 1 "$BLOCKS"); do
  nigiri faucet "$ADDRESS" >/dev/null
  printf "  block %d/%d\n" "$i" "$BLOCKS"
done

END_HEIGHT=$(nigiri rpc getblockcount)
echo "Done. Chain height: $START_HEIGHT -> $END_HEIGHT"
