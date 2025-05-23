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

Once the server is started, it exposes an interactive openapi documentation at `http://localhost:8080` by default. Explore it from your browser!

Fuzzy matching queries should use `GET /codes/top?collection={collection}&num={top}&query={query}`.

```shell
# example
➜ curl -s 'http://localhost:8080/codes/top?collection=cell_type&query=kocyte&num=3' | jq

{
  "codes": [
    {
      "label": "leukocyte",
      "uri": "<http://purl.obolibrary.org/obo/CL_0000738>",
      "score": null
    },
    {
      "label": "myeloid leukocyte",
      "uri": "<http://purl.obolibrary.org/obo/CL_0000766>",
      "score": null
    },
    {
      "label": "leukocyte migration",
      "uri": "<http://purl.obolibrary.org/obo/GO_0050900>",
      "score": null
    }
  ]
}
}
```

To discover available collections, use `GET /list`.

```shell
# example
$ curl 'http://localhost:8080/collections'
{
  "collections": ["cell_type","source_material","taxon_id"]
}
```

## Example

Here is a minimal example of how fuzon-http may be used from a tool.
It is a bash script that continuously reads user-input, retrieves the top 10 best matching codes from the server and displays them in the terminal.

```bash
#!/bin/bash
URL=http://localhost:8080
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
  curl -s "${URL}/codes/top?query=${keys}&num=10&collection=cell_type" |
    jq '{codes: .codes | map({(.label): .uri})} | .codes | add'
  # move cursor up 13 lines (1 for input display + 10 codes + 2 braces)
  tput cuu 13
done
```

And here it is in action:

![](../../docs/img/fuzon-http.gif)
