set positional-arguments
set shell := ["bash", "-cue"]
image := "ghcr.io/sdsc-ordes/fuzon"
root := justfile_directory()

## build

# build all packages
build *args:
  cargo build \
    --manifest-path fuzon/Cargo.toml \
    --release
  maturin build \
    --manifest-path pyfuzon/Cargo.toml \
    --release \
    {{args}}

# install editable python package
maturin-dev *args:
  maturin develop \
  --manifest-path pyfuzon/Cargo.toml \
  --uv \
  {{args}}

## development

# enter nix devshell
develop-nix *args:
  cd {{root}} \
    && cmd=("$@") \
    && { [ -n "${cmd:-}" ] || cmd=("zsh"); } \
    && nix develop ./tools/nix#default --command "${cmd[@]}"

# enter development container
develop-docker:
  docker run \
    --user 1000:1000 \
    -it \
    -w "/build/workspace" \
    --mount type=bind,source="$(pwd)",target=/build/work \
    {{image}}:dev

## maintenance

# build images
docker-build:
  nix build -L "./tools/nix#image.dev" --out-link "target/image.dev" \
    && docker load < "build/image.dev"
  nix build -L "./tools/nix#image.fuzon" --out-link "target/image.fuzon" \
    && docker load < "build/image.fuzon"

# push images
docker-push: docker-build
  docker push {{image}}:dev
  docker push {{image}}:fuzon


