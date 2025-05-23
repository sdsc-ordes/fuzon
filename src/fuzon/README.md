# fuzon

Binary and core rust library crate.

## Installation

fuzon can be built with cargo.

```shell
git clone https://github.com/sdsc-ordes/fuzon
cd fuzon
cargo build --release

./target/release/fuzon --help
```

## Usage

### Command Line Interface

To filter the top 3 matches in a file non-interactively:

```shell
$ fuzon -q 'aspirin' --top 3 -s onto1.ttl -s onto2.ttl
```

Running fuzon without a query will start an interactive prompt to browse the input ontologies.

### Rust Library

`TermMatcher` is the central struct of fuzon. It stores a collection of `Term`s, representing label-URI pairs and exposes method to query these `Terms` with text.

```rust
use fuzon;

let mut sources = vec![
  "./onto1.ttl".to_string(),
  "https://example.org/onto2.xml".to_string(),
];
// all ontologies combined into a single graph
let matcher = TermMatcher::from_paths(&sources);
matcher.rank_terms("some query");
```

`TermMatcher` also supports a caching mechanism via serde and postcard:

```rust
// dump/load a single cache entry with combined sources
let path = get_cache_path(&mut sources);
matcher.dump(&path);
let matcher2 = TermMatcher::load(&path);
```

Ontologies can also be cached individually to reduce cache redundancy at the cost of slower load times:

```rust
use fuzon::cache;

cache::cache_by_source(&sources);
// load one cache entry per source -> merge
let matcher3 = cache::load_by_source(&sources);
```
