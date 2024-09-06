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

nix-develop *args:
  cd {{root}} && \
  cmd=("$@") && \
  { [ -n "${cmd:-}" ] || cmd=("zsh"); } && \
  nix develop ./tools/nix#default --command "${cmd[@]}"
