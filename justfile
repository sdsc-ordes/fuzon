set positional-arguments
set shell := ["bash", "-cue"]
root := justfile_directory()


# build wheel
build *args:
  maturin build \
    --release \
    {{args}}

# development environment
develop *args:
  maturin develop \
  --uv \
  {{args}}
