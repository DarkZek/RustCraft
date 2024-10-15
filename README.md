# Rustcraft

Rustcraft is a Voxel Game with the goal of providing a fun, new and feature packed experience for sandbox lovers.

## Installation

Right now there are no stable releases, you have to build it yourself.

### Build

##### Client (Native)

TODO

##### Client (WASM)

```bash
cd ./site && docker compose up
```

##### Server

```bash
cd ./server && docker compose up
```

##### API

```bash
cd ./api && docker compose up
```

### Develop

##### Client (Native)

TODO

##### Client (WASM)

```bash
cargo run --bin rc_client --target wasm32-unknown-unknown
```

##### Server

```bash
cargo run --bin rc_server
```

##### API

```bash
cd ./api && cargo run
```
