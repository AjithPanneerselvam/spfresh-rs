# spfresh-rs

`spfresh-rs` provides Rust bindings for the C++ [SPFresh](https://github.com/SPFresh/SPFresh) library, the implementation of [SPFresh: Incremental In-Place Update for Billion-Scale Vector Search](https://arxiv.org/abs/2410.14452).

## Build

Requires [SPFresh](https://github.com/SPFresh/SPFresh) and the [PtilopsisL/rocksdb](https://github.com/PtilopsisL/rocksdb) fork built and installed.

```bash
cp .env.example .env
# edit .env with your paths

just build
```

Or with Cargo directly:

```bash
export SPFRESH_ROOT=/path/to/SPFresh
export ROCKSDB_ROOT=/path/to/rocksdb-install
cargo build
```

## Roadmap

| Feature | Status |
|---------|--------|
| BKT with `f32` vectors: create, build, search, free | In progress |
| KDT in-memory index | Planned |
| SPANN disk-backed index | Planned |
| Save and load | Planned |
| In-memory add, delete, and update | Planned |
| Batch search | Planned |
| SPFresh incremental streaming updates | Planned |
| `i8` and `u8` vectors | Planned |

## License

MIT
