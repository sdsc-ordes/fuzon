# fuzon

> [!WARNING]
> This repository is a prototype and not yet in a usable state.

fzf for browsing ontologies

## installation

```shell
git clone https://github.com/sdsc-ordes/fuzon
cd fuzon
cargo build --release

./target/release/fuzon --help
```

## usage

### Command line interface

Running fuzon with a set of RDF ontologies / terminologies will start an interactive prompt using [fzf](https://github.com/junegunn/fzf) to browse the input ontologies.

```shell
$ fuzon -i onto1.ttl -i onto2.ttl
```

### rust crate

### python package

