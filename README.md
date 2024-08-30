# fuzon

> [!WARNING]
> This repository is a prototype and not yet in a usable state.

fuzon allows to search entities in rdf knowledge graphs based on their labels. It is a wrapper around the [rff](https://github.com/stewart/rff) fuzzy finder. Example use cases of this tool include finding instances belonging to an enumeration class in a given source ontology. It prefetches URI - label pairs using SPARQL queries in the back-end, ran on an in-memory [oxigraph](https://github.com/oxigraph/oxigraph) store to find the items relevant to index. This index allows for the highly performant, near real-time feedback to "auto-complete" in the terminal.


## installation

```shell
git clone https://github.com/sdsc-ordes/fuzon
cd fuzon
cargo build --release

./target/release/fuzon --help
```

## usage

### Command line interface



To filter the top 3 matches in a file non-interactively:

```shell
$ fuzon -q 'aspirin' --top 3 -s onto1.ttl -s onto2.ttl
```


Not implemented yet: ~~Running fuzon with a set of RDF ontologies / terminologies will start an interactive prompt using [fzf](https://github.com/junegunn/fzf) to browse the input ontologies.~~

### rust crate
```rust
use fuzon;
let r1 = BufReader::new(File::open("onto1.ttl")) 
let r2 = BufReader::new(File::open("onto2.ttl"))
// all readers combined into a single graph
let matcher = TermMatcher::from_readers(vec![r1, r2])
matcher.rank_terms("some query")
```


### python package

