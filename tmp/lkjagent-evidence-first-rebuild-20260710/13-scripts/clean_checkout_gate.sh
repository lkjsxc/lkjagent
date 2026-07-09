#!/bin/sh
set -eu

repo=${1:-.}
repo=$(cd "$repo" && pwd)

cd "$repo"

if test -n "$(git status --porcelain)"; then
  echo "worktree must be clean" >&2
  exit 1
fi

git ls-files --error-unmatch Cargo.lock >/dev/null
git ls-files --error-unmatch Dockerfile >/dev/null
git ls-files --error-unmatch docker-compose.yml >/dev/null

out=$(mktemp -d "${TMPDIR:-/tmp}/lkjagent-clean-checkout.XXXXXX")
trap 'rm -rf "$out"' EXIT HUP INT TERM
git archive --format=tar --output "$out/repository.tar" HEAD
tar -xf "$out/repository.tar" -C "$out"
rm "$out/repository.tar"

cd "$out"
test -f Cargo.lock
export HOME="$out/home"
export COMPOSE_PROJECT_NAME="lkjagent_clean_$$"
unset CARGO_TARGET_DIR RUSTC_WRAPPER RUSTFLAGS COMPOSE_FILE COMPOSE_PROFILES
mkdir -p "$HOME"
docker compose --profile verify build --no-cache verify test lint
docker compose --profile verify run --rm verify
docker compose --profile verify run --rm test
docker compose --profile verify run --rm lint

test -z "$(git -C "$repo" status --porcelain)"

echo "PASS clean checkout $(git -C "$repo" rev-parse HEAD)"
