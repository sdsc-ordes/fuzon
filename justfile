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

# development environment
py-develop *args:
  maturin develop \
  --manifest-path pyfuzon/Cargo.toml \
  --uv \
  {{args}}
