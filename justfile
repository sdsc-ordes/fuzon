set positional-arguments
set shell := ["bash", "-cue"]
root := justfile_directory()


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

develop-nix *args:
  cd {{root}} \
    && cmd=("$@") \
    && { [ -n "${cmd:-}" ] || cmd=("zsh"); } \
    && nix develop ./tools/nix#default --command "${cmd[@]}"

develop-docker:
  docker run \
    --user 1000:1000 \
    -it \
    -w "/build/workspace" \
    --mount type=bind,source="$(pwd)",target=/build/work \
    {{image}}:dev

# maintenance

image-build:
  nix build -L "./tools/nix#image.dev" --out-link "target/image.dev" \
    && docker load < "build/image.dev"
  nix build -L "./tools/nix#image.fuzon" --out-link "target/image.fuzon" \
    && docker load < "build/image.fuzon"



