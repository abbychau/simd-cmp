# SIMD-Accelerated File Comparison

This project implements a file comparison utility similar to Linux/macOS `cmp`, but using SIMD instructions from Rust's standard library to accelerate the comparison.

## Requirements

- Rust nightly (needed for `std::simd`)
- Standard build tools

## Features

- SIMD-accelerated file comparison for better performance using Rust's std::simd
- Command-line interface similar to the standard `cmp` utility
- Comprehensive benchmarks to demonstrate performance gains

## Running the Application

```bash
cargo run -- file1 file2       # Compare two files
cargo run -- file1 file2 -s    # Silent mode - only return exit code
```

