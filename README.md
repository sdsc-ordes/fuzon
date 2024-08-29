# fuzon

> [!WARNING]
> This repository is a prototype and not yet in a usable state.

fuzon is a tool which lets you interactively prompt RDF graphs using a fuzzy finder [fzf](https://github.com/junegunn/fzf). Example use cases of this tool include finding instances belonging to an enumeration class in a given source ontology. It uses SPARQL queries in the back-end to find the items relevant to index. This index allows for the highly performant, near real-time feedback to "auto-complete" in the terminal.

```shell
>Bla    # <- user types this
Bladder cancer (obo:xyz) <- top n hits updated on each keystroke
Bladder infection (obo:abc)
Bland taste (obo:lol)
```

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

