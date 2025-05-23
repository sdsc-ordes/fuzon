set positional-arguments
set shell := ["bash", "-cue"]
root_dir := `git rev-parse --show-toplevel`

# Install editable python package.
maturin-dev *args:
  cd {{root_dir}} && \
    maturin develop \
      --manifest-path pyfuzon/Cargo.toml \
      --uv \
      {{args}}
