set positional-arguments
set shell := ["bash", "-cue"]
root_dir := `git rev-parse --show-toplevel`
image := "ghcr.io/sdsc-ordes/fuzon"
ctr := "docker"

# Default recipe to list all recipes.
[private]
default:
  just --list image --no-aliases

# Build images.
build:
  cd {{root_dir}} && \
    nix build -L "./tools/nix#images.dev" --out-link "target/image.dev" && \
    {{ctr}} load < "target/image.dev"
  cd {{root_dir}} && \
    nix build -L "./tools/nix#images.fuzon" --out-link "target/image.fuzon" && \
    {{ctr}} load < "target/image.fuzon"

# Push images.
push: build
  {{ctr}} push {{image}}:dev
  {{ctr}} push {{image}}:fuzon

run *args:
  cd {{root_dir}} && \
    {{ctr}} run \
      {{args}} \
      {{image}}:dev
