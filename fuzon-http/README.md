# fuzon-http

This is a web-server to deploy fuzon as a web-service.
All ontologies are loaded once on server startup, and the indices are kept in memory.

## Configuration

The server takes a configuration file as input to determine what ontologies to load, and which collections to load them into. Collections are individual matchers which can be queried independently.

## Installation

```shell
cd fuzon-http
cargo build --release
```

## Usage

Start the server with:

```shell
../target/release/fuzon-http --config config/example.json
```

Fuzzy matching queries should use `GET /top?collection={collection}&top={top}&query={query}`.

To discover available collections, use `GET /list`.

