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

fuzon helps you **quickly find relevant entities** (URIs) based on text. It does so by fuzzy matching inputs against the annotations attached to concepts in an RDF graph, allowing for partial matches and typos.

The goal of fuzon is to **accelerate exploration of complex ontologies** or terminologies to make semantic data more accessible to users. It can be used directly as a command line tool, deployed as a web service, or integrated into other tools as a (rust or python) library.

<div align="center">
  <a href="https://asciinema.org/a/rg5bfeXmKrXjwNuLCUUnmttpL">
    <img src="https://media.githubusercontent.com/media/sdsc-ordes/fuzon/refs/heads/main/docs/img/fuzon-cli.gif" alt="fuzon cli gif" width="66%" />
  </a>
</div>

## Under the Hood

fuzon parses input ontologies from local files or URLs and indexes URI - label pairs. This allows for highly performant fuzzy searches, with near-instant feedback to use in "auto-complete" interfaces. Previously loaded ontologies are also cached on disk to speed-up subsequent runs. The fuzzy search relies on the [rff](https://github.com/stewart/rff) fuzzy finder which itself uses the [algorithm from fzy](https://github.com/jhawthorn/fzy/blob/master/ALGORITHM.md), a variant of [Needleman-Wunsch](https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm). 

## Documentation

Installation and usage instructions are available for the individual crates:

* [:crab: Rust command line tool and library](./src/fuzon/README.md)
* [:snake: Python library](./src/pyfuzon/README.md)
* [:spider: Web server](./src/fuzon-http/README.md)

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


