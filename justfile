set positional-arguments
set shell := ["bash", "-cue"]
image := "ghcr.io/sdsc-ordes/fuzon"
root := justfile_directory()

## Build

# Build all packages.
build *args:
  cargo build \
    --manifest-path fuzon/Cargo.toml \
    --release
  maturin build \
    --manifest-path pyfuzon/Cargo.toml \
    --release \
    {{args}}

# Install editable python package.
maturin-dev *args:
  maturin develop \
  --manifest-path pyfuzon/Cargo.toml \
  --uv \
  {{args}}

package-nix *args:
    cd {{root}} && \
    nix build "./tools/nix#fuzon" -o "package/fuzon" {{args}}

## Development

# Enter nix devshell.
develop-nix *args:
  cd {{root}} \
    && cmd=("$@") \
    && { [ -n "${cmd:-}" ] || cmd=("zsh"); } \
    && nix develop ./tools/nix#default --command "${cmd[@]}"

# Enter development container
develop-docker:
  docker run \
    --user 1000:1000 \
    -it \
    -w "/build/workspace" \
    --mount type=bind,source="$(pwd)",target=/build/work \
    {{image}}:dev

## Maintenance

# Build images.
docker-build:
  nix build -L "./tools/nix#images.dev" --out-link "target/image.dev" \
    && docker load < "target/image.dev"
  nix build -L "./tools/nix#images.fuzon" --out-link "target/image.fuzon" \
    && docker load < "target/image.fuzon"

# Push images.
docker-push: docker-build
  docker push {{image}}:dev
  docker push {{image}}:fuzon
