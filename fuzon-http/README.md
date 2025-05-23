# fuzon-http

This is a web-server to deploy fuzon as a web-service.
All ontologies are loaded once on server startup, and the indices are kept in memory.

## Configuration

The server takes a configuration file as input to determine what ontologies to load, and which collections to load them into. Collections are individual matchers which can be queried independently.

## Installation

fuzon-http can be built with cargo.

```shell
git clone https://github.com/sdsc-ordes/fuzon
cd fuzon
cargo build --release

./target/release/fuzon-http --config ./fuzon-http/config/example.json
```

## Usage

Start the server with:

```shell
../target/release/fuzon-http --config config/example.json
```

Fuzzy matching queries should use `GET /top?collection={collection}&top={top}&query={query}`.

```shell
# example
$ curl 'http://localhost:8080/top?collection=cell_type&top=3&query=leukocyte'
{
  "codes": [
    {
      "label":"\"myeloid leukocyte\"",
      "uri":"<http://purl.obolibrary.org/obo/CL_0000766>",
      "score":null
    },
    {
      "label":"\"nongranular leukocyte\"",
      "uri":"<http://purl.obolibrary.org/obo/CL_0002087>",
      "score":null
    },
    {
      "label":"\"myeloid leukocyte migration\"",
      "uri":"<http://purl.obolibrary.org/obo/GO_0097529>",
      "score":null
    }
  ]
}
```

To discover available collections, use `GET /list`.

```shell
# example
$ curl 'http://localhost:8080/list'
{
  "collections": ["cell_type","source_material","taxon_id"]
}
```

## Example

Here is a minimal example of how fuzon-http may be used from a tool.
It is a bash script that continuously reads user-input, retrieves the top 10 best matching codes from the server and displays them in the terminal.

```bash
#!/bin/bash
keys=""
while IFS= read -r -n1 -s key; do
  # delete chars when backspace is pressed
  if [[ $key == $'\x7f' ]]; then
    keys="${keys%?}"
  else
    keys="${keys}${key}"
  fi
  # Clear terminal ouptut
  tput ed
  echo "input: " $keys
  curl -s "http://localhost:8080/top?query=${keys}&top=10&collection=cell_type" | jq -r '.codes[] | "\(.label) \(.uri)"'
  # move cursor up 11 lines (1 for input display + 10 codes)
  tput cuu 11
done
```

And here it is in action:

![](../docs/img/fuzon-http.svg)
