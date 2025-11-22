#!/usr/bin/env bash
set -euo pipefail

# Automated build and deployment for the pSOL Anchor workspace.
# Usage: CLUSTER=devnet ./scripts/auto_deploy.sh

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKDIR="$ROOT_DIR"
CLUSTER="${CLUSTER:-devnet}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "[error] Missing required tool: $1" >&2
    exit 1
  fi
}

validate_cluster() {
  case "$CLUSTER" in
    devnet|testnet|mainnet-beta) ;;
    *)
      echo "[error] Unsupported cluster '$CLUSTER'. Use devnet, testnet, or mainnet-beta." >&2
      exit 1
      ;;
  esac
}

require_command anchor
require_command solana
validate_cluster

if [[ -z "${ANCHOR_WALLET:-}" ]]; then
  echo "[error] ANCHOR_WALLET must be set to a funded keypair for $CLUSTER deployments." >&2
  exit 1
fi

pushd "$WORKDIR" >/dev/null

echo "[info] Building workspace..."
anchor build

echo "[info] Deploying to cluster: $CLUSTER"
anchor deploy --provider.cluster "$CLUSTER"

echo "[info] Deployment complete."
popd >/dev/null
