# fuse-rs

A lightweight fuzzy-search library for Rust, with bindings for WebAssembly and Node.js.
This is a Rust port of the popular [Fuse.js](https://fusejs.io/) JavaScript library.

## Project Structure

This project is organized as a Rust workspace with multiple crates:

- `fuse-rs`: Core library implemented in Rust
- `fuse-wasm`: WebAssembly bindings using wasm-bindgen
- `fuse-node`: Node.js bindings using Neon

## Core Library Usage (Rust)

```rust
use fuse_rs::{Fuse, FuseOptions};
use serde_json::json;

fn main() {
    // Sample data
    let data = vec![
        json!({"title": "Old Man's War", "author": "John Scalzi"}),
        json!({"title": "The Lock Artist", "author": "Steve Hamilton"}),
    ];

    // Create options
    let options = FuseOptions::default()
        .set_include_score(true)
        .set_keys(vec!["title", "author"]);

    // Create Fuse instance
    let fuse = Fuse::new(&data, &options);

    // Search
    let results = fuse.search("old");
    println!("{:?}", results);
}
```

## WebAssembly Usage

```javascript
import { WasmFuse } from 'fuse-rs-wasm';

// Sample data
const data = [
  { title: "Old Man's War", author: "John Scalzi" },
  { title: "The Lock Artist", author: "Steve Hamilton" }
];

// Options
const options = {
  keys: ['title', 'author'],
  includeScore: true
};

// Create Fuse instance
const fuse = new WasmFuse(data, options);

// Search
const results = fuse.search("old");
console.log(results);
```

## Node.js Usage

```javascript
const Fuse = require('fuse-rs-node');

// Sample data
const data = [
  { title: "Old Man's War", author: "John Scalzi" },
  { title: "The Lock Artist", author: "Steve Hamilton" }
];

// Options
const options = {
  keys: ['title', 'author'],
  includeScore: true
};

// Create Fuse instance
const fuse = new Fuse(data, options);

// Search
const results = fuse.search("old");
console.log(results);

// Clean up (optional)
fuse.destroy();
```

## Building

### Core Library

```bash
cargo build --release -p fuse-rs
```

### WebAssembly

```bash
cd crates/fuse-wasm
wasm-pack build --target web --out-dir dist
```

### Node.js

```bash
cd crates/fuse-node
npm install
npm run build
```

## License

MIT
