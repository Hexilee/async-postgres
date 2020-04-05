<div align="center">
  <h1>async-postgres</h1>
  <p><strong>A runtime-independent, asynchronous PostgreSQL client.</strong> </p>
  <p>

[![Stable Test](https://github.com/Hexilee/async-postgres/workflows/Stable%20Test/badge.svg)](https://github.com/Hexilee/async-postgres/actions)
[![codecov](https://codecov.io/gh/Hexilee/async-postgres/branch/master/graph/badge.svg)](https://codecov.io/gh/Hexilee/async-postgres) 
[![Rust Docs](https://docs.rs/async-postgres/badge.svg)](https://docs.rs/async-postgres)
[![Crate version](https://img.shields.io/crates/v/async-postgres.svg)](https://crates.io/crates/async-postgres)
[![Download](https://img.shields.io/crates/d/async-postgres.svg)](https://crates.io/crates/async-postgres)
[![MSRV-1.40](https://img.shields.io/badge/MSRV-1.40-blue.svg)](https://blog.rust-lang.org/2019/12/19/Rust-1.40.0.html)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Hexilee/async-postgres/blob/master/LICENSE)

  </p>
</div>
<br>

This crate is an out-of-the-box wrapper of [tokio-postgres](https://crates.io/crates/tokio-postgres).

### Pros

- runtime-independent, can be used on any async runtime.
- build-in tls support, based on [tokio-rustls](https://github.com/quininer/tokio-rustls).

### Performance

### Develop
