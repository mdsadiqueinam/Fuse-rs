# Code Organization in fuse-rs

This document describes the overall organization and structure of the fuse-rs project.

## Workspace Structure

The project is organized as a Rust workspace with multiple crates:

- **fuse-rs**: Core library implementing the fuzzy search functionality
- **fuse-wasm**: WebAssembly bindings for using the library in web browsers
- **fuse-node**: Node.js bindings for using the library in Node.js applications

## Core Library Structure

The core `fuse-rs` library is organized into several key modules:

- **core**: Main functionality of the fuzzy search engine
  - **fuse**: Primary search implementation
  - **options**: Configuration for search behavior
  - **results**: Types for representing search results
  - **compute_score**: Scoring algorithms
  
- **helpers**: Utility functions and traits
  - **get**: Path-based property access
  - **diacritics**: Accent/diacritic mark handling
  
- **tools**: Supporting infrastructure
  - **key_store**: Management of searchable fields
  - **norm**: Field length normalization
  - **fuse_index**: Search indexing

## Code Structure Conventions

Throughout the codebase, we follow these conventions:

### File Organization

Each file is structured with the following sections:

1. Module documentation (`//!`) at the top
2. Imports, grouped by source (standard library, external, internal)
3. Types & Constants
4. Public API
5. Implementation Details
6. Tests

Code sections are separated with a standard divider:

```rust
//----------------------------------------------------------------------
// Section Name
//----------------------------------------------------------------------
```

### Documentation

- All public APIs have documentation comments with examples where appropriate
- Types document their purpose and usage
- Functions document parameters and return values
- Complex behavior is explained with examples

### Naming Conventions

- Types follow `PascalCase`
- Functions and variables use `snake_case`
- Constants use `SCREAMING_SNAKE_CASE`
- Type parameters use single capital letters (e.g., `T`, `K`, `V`)

## Key Components

### Fuse

The main entry point for fuzzy search functionality. Creates and manages the search index, and exposes methods for performing searches.

### FuseOptions

Configures the behavior of the search engine, including:
- Case sensitivity
- Diacritic handling
- Threshold for matching
- Keys to search in
- Sorting behavior

### KeyStore

Manages the collection of keys that define which fields in documents are searchable and how they're weighted in relevance calculations.

### Search Results

Search operations return results that can include:
- The original item that matched
- The calculated relevance score
- Details about which fields matched and where

## Future Improvements

Areas that could benefit from future improvement:
- Implementation of the full fuzzy search algorithm
- More comprehensive indexing for better performance
- Additional test coverage for edge cases
- Expand WebAssembly and Node.js bindings functionality
- Add more examples for different use cases
