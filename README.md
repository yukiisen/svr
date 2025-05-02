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

## Benchmark
This was tested on the introduction page of the svelte Documentation.

```sh
⋊> ~ autocannon 127.0.0.1:3000/svelte.html                                          21:15:46
Running 10s test @ http://127.0.0.1:3000/svelte.html
10 connections


┌─────────┬────────┬─────────┬─────────┬─────────┬────────────┬────────────┬─────────┐
│ Stat    │ 2.5%   │ 50%     │ 97.5%   │ 99%     │ Avg        │ Stdev      │ Max     │
├─────────┼────────┼─────────┼─────────┼─────────┼────────────┼────────────┼─────────┤
│ Latency │ 167 ms │ 2382 ms │ 4677 ms │ 4753 ms │ 2424.76 ms │ 1332.57 ms │ 4811 ms │
└─────────┴────────┴─────────┴─────────┴─────────┴────────────┴────────────┴─────────┘
┌───────────┬─────────┬─────────┬────────┬────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%    │ 97.5%  │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼────────┼────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 1,040   │ 1,040   │ 1,359  │ 1,429  │ 1,322.4 │ 118.61  │ 1,040   │
├───────────┼─────────┼─────────┼────────┼────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 83.8 MB │ 83.8 MB │ 110 MB │ 115 MB │ 107 MB  │ 9.56 MB │ 83.8 MB │
└───────────┴─────────┴─────────┴────────┴────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 10

26k requests in 10.08s, 1.07 GB read
6 errors (0 timeouts)
```

## Security

This server does not sandbox or restrict access — be careful when exposing it to the internet!

## TODO

* [ ] Directory listing toggle
* [ ] Basic access logging
* [ ] Graceful shutdown
* [ ] SPA Support
