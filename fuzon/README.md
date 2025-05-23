# pyfuzon

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

```rust
use fuzon;
let onto1 = "./onto1.ttl".to_string()
let onto2 = "https://example.org/onto2.xml".to_string()
// all ontologies combined into a single graph
let matcher = TermMatcher::from_files(vec![onto1, onto2])
matcher.rank_terms("some query")
```
