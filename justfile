set positional-arguments
set shell := ["bash", "-cue"]
root := justfile_directory()

# Default recipe to list all recipes.
default:
  just --list

# Build all packages.
build *args:
  cargo build \
    --manifest-path src/fuzon/Cargo.toml \
    --release
  maturin build \
    --manifest-path src/pyfuzon/Cargo.toml \
    --release \
    {{args}}

# Package rust binary with nix.
package-nix *args:
    cd {{root}} && \
    nix build "./tools/nix#fuzon" -o "package/fuzon" {{args}}

# Enter nix devshell.
develop-nix *args:
  cd {{root}} \
    && cmd=("$@") \
    && { [ -n "${cmd:-}" ] || cmd=("zsh"); } \
    && nix develop ./tools/nix#default --command "${cmd[@]}"

# Enter development container
develop-docker:
  just image::run \
    -it \
    --user 1000:1000 \
    -w "/workspace" \
    --mount type=bind,source="$(pwd)",target=/workspace


# manage OCI container images
mod image './tools/just/image.just'
# manage Python package
mod maturin './tools/just/maturin.just'
