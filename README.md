<p align="center">
  <img src="./docs/img/fuzon.svg" alt="fuzon logo" width="250">
</p>

<p align="center">
</p>
<p align="center">
  <a href="https://github.com/sdsc-ordes/fuzon/releases/latest">
    <img src="https://img.shields.io/github/release/sdsc-ordes/fuzon.svg?style=for-the-badge" alt="Current Release label" /></a>
  <a href="https://github.com/sdsc-ordes/fuzon/actions/workflows/maturin.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/sdsc-ordes/fuzon/maturin.yaml?label=tests&style=for-the-badge" alt="Test Status label" /></a>
  <a href="https://sdsc-ordes.github.io/modos-api">
    <img src="https://img.shields.io/website?url=https%3A%2F%2Fsdsc-ordes.github.io%2Ffuzon&up_message=online&up_color=blue&down_message=offline&style=for-the-badge&label=docs" alt="Documentation website" /></a>
  <a href="http://www.apache.org/licenses/LICENSE-2.0.html">
    <img src="https://img.shields.io/badge/LICENSE-Apache2.0-ff69b4.svg?style=for-the-badge" alt="License label" /></a>
</p>

# fuzon

fuzon helps you quickly find relevant entities (URIs) based on text. It does so by fuzzy matching inputs against the annotations attached to concepts in an RDF graph, allowing for partial matches and typos.

The goal of fuzon is to accelerate exploration of complex ontologies or terminologies to make semantic data more accessible to users. It can be used directly as a command line too, embedded as a webserver, or integrated into other tools as a (rust or python) library.


## Principles 

fuzon is built around the [rff](https://github.com/stewart/rff) fuzzy finder which itself uses the [algorithm from fzy](https://github.com/jhawthorn/fzy/blob/master/ALGORITHM.md) a variant of [Needleman-Wunsch](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm). In addition, fuzon prefetches URI - label pairs by parsing source ontologies (either from a local file or a URL). This allows for highly performant fuzzy searches, with near-instant feedback to use in "auto-complete" interfaces. Previously loaded ontologies are also cached to speed-up subsequent runs.

## Documentation

Installation and usage instructions are available for the individual crates:

* [:crab: Rust command line tool and library](./fuzon/README.md)
* [:snake: Python library](./pyfuzon/README.md)
* [:spider: Web server](./fuzon-http/README.md)

## Installation

### Command Line Tool

The rust crate can be installed by cloning the repo and building locally:

```shell
git clone https://github.com/sdsc-ordes/fuzon
cd fuzon
cargo build --release

./target/release/fuzon --help
```

### Web server

```shell
git clone https://github.com/sdsc-ordes/fuzon
cd fuzon
cargo build --release

./target/release/fuzon-http --config ./fuzon-http/config/example.json
```

## Usage

### Command Line Interface

To filter the top 3 matches in a file non-interactively:

```shell
$ fuzon -q 'aspirin' --top 3 -s onto1.ttl -s onto2.ttl
```

Running fuzon without a query will start an interactive prompt to browse the input ontologies.

### Rust Library
```rust
use fuzon;
let onto1 = "./onto1.ttl".to_string()
let onto2 = "https://example.org/onto2.xml".to_string()
// all ontologies combined into a single graph
let matcher = TermMatcher::from_files(vec![onto1, onto2])
matcher.rank_terms("some query")
```

## Development

A nix dev shell with all build dependencies is provided.
Assuming just and nix are installed on the machine, you can enter the shell with:

```shell
just develop-nix
```

Alternatively, docker can be used as a development shell:

```shell
just develop-docker
```

Once inside a development shell, the python+rust packages can be built with:

```shell
just build
```

Or the python bindings can be installed in editable mode using:

```shell
just maturin-dev
# pyfuzon now available in python shells
```


