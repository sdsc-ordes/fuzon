# fuzon

> [!WARNING]
> This repository is a prototype and not yet in a usable state.

fuzon allows to fuzzy search entities in rdf knowledge graphs based on their labels. It is a wrapper around the [rff](https://github.com/stewart/rff) fuzzy finder. Example use cases of this tool include finding codes based on their label in a given source ontology. It prefetches URI - label pairs in the back-end, by parsing source ontologies (either from a local file or a URL). This allows for highly performant fuzzy searches, with near-instant feedback to use in "auto-complete" interfaces.

## installation

The rust crate can be installed by cloning the repo and building locally:

```shell
git clone https://github.com/sdsc-ordes/fuzon
cd fuzon
cargo build --release

./target/release/fuzon --help
```

The python package is distributed on PyPI and can be installed with:

```shell
pip install pyfuzon
```

## usage

### command line interface

To filter the top 3 matches in a file non-interactively:

```shell
$ fuzon -q 'aspirin' --top 3 -s onto1.ttl -s onto2.ttl
```

Running fuzon without a query will start an interactive prompt to browse the input ontologies.

### rust library
```rust
use fuzon;
let onto1 = "./onto1.ttl".to_string()
let onto2 = "https://example.org/onto2.xml".to_string()
// all ontologies combined into a single graph
let matcher = TermMatcher::from_files(vec![onto1, onto2])
matcher.rank_terms("some query")
```

### python package

```python
from pyfuzon.matcher import TermMatcher

matcher = TermMatcher.from_files("https://example.org/onto1.ttl", "/data/onto2.ttl")
matcher.terms #accesses the list of terms loaded from input files
matcher.score("query") # returns the match score of each term for the input query.
matcher.rank("query") # returns the list of terms sorted by similarity with the query.
matcher.top("query", 5) # shows top 5 most similar results (sorted).
```

## development

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


