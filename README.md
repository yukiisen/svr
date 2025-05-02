# svr

**`svr`** is a tiny, static HTTP server written in Rust. No dependencies, Everything is built from scratch..
## Features

- Minimal and fast?
- Low memory usage (Almost 3MB on VoidLinux)
- Serves static files from any directory
- Custom host and port

## Installation

Build from source:

```bash
git clone https://github.com/yourusername/svr
cd svr
make install clean
```

## Usage

```bash
svr [--port <port>] [--host <host>] [path]
```

### Options

| Flag     | Description        | Default     |
| -------- | ------------------ | ----------- |
| `--port` | Port to listen on  | `3000`      |
| `--host` | Host to bind to    | `127.0.0.1` |
| `path`   | Directory to serve | Current dir |

### Examples

```bash
# Serve current dir at 127.0.0.1:3000
svr

# Serve ./public at port 8080
svr --port 8080 ./public

# Serve from root and listen on all interfaces
svr --host 0.0.0.0 /
```

## Security

This server does not sandbox or restrict access — be careful when exposing it to the internet!

## TODO

* [ ] Directory listing toggle
* [ ] Basic access logging
* [ ] Graceful shutdown
* [ ] SPA Support
