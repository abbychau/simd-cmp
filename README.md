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

## Performance

The SIMD-accelerated file comparison is significantly faster than the standard `cmp` utility, especially for larger files. Benchmarks show a performance improvement of roughly 4x for files larger than 1MB.

```
~/simd-cmp/target/release > hyperfine 'cmp a2.txt b2.txt' --warmup 1 
Benchmark 1: cmp a2.txt b2.txt
  Time (mean ± σ):      1.969 s ±  0.024 s    [User: 1.779 s, System: 0.157 s]
  Range (min … max):    1.945 s …  2.021 s    10 runs
 
~/simd-cmp/target/release > hyperfine './simd-cmp a2.txt b2.txt' --warmup 1
Benchmark 1: ./simd-cmp a2.txt b2.txt
  Time (mean ± σ):     542.2 ms ±  51.9 ms    [User: 105.5 ms, System: 403.4 ms]
  Range (min … max):   507.1 ms … 682.4 ms    10 runs
 
~/simd-cmp/target/release > hyperfine 'cmp a.txt b.txt' --warmup 1       
Benchmark 1: cmp a.txt b.txt
  Time (mean ± σ):     668.5 ms ±  10.8 ms    [User: 592.1 ms, System: 53.3 ms]
  Range (min … max):   656.5 ms … 685.6 ms    10 runs
 
~/simd-cmp/target/release > hyperfine './simd-cmp a.txt b.txt' --warmup 1 
Benchmark 1: ./simd-cmp a.txt b.txt
  Time (mean ± σ):     179.6 ms ±   3.1 ms    [User: 35.8 ms, System: 133.2 ms]
  Range (min … max):   175.9 ms … 186.2 ms    15 runs
```
