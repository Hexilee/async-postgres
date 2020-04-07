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

This crate is a wrapper of [tokio-postgres](https://crates.io/crates/tokio-postgres).

### Pros

Runtime-independent, can be used on any async runtime.

### Usage

Almost the same with tokio-postgres.

- TCP or UDS

```rust
use async_postgres::connect;
use std::error::Error;
use async_std::task::spawn;

async fn play() -> Result<(), Box<dyn Error>> {
    let url = "host=localhost user=postgres";
    let (client, conn) = connect(url.parse()?).await?;
    spawn(conn);
    let row = client.query_one("SELECT * FROM user WHERE id=$1", &[&0]).await?;
    let value: &str = row.get(0);
    println!("value: {}", value);
    Ok(())
}
```

- TLS

```rust
use async_postgres::connect_tls;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use std::fs;
use std::error::Error;
use async_std::task::spawn;

async fn play() -> Result<(), Box<dyn Error>> {
    let cert = fs::read("database_cert.pem")?;
    let cert = Certificate::from_pem(&cert)?;
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .build()?;
    let connector = MakeTlsConnector::new(connector);
    let url = "host=localhost user=postgres sslmode=require";
    let (client, conn) = connect_tls(url.parse()?, connector).await?;
    spawn(conn);
    let row = client.query_one("SELECT * FROM user WHERE id=$1", &[&0]).await?;
    let value: &str = row.get(0);
    println!("value: {}", value);
    Ok(())
}
```

### Performance

Almost the same with tokio-postgres, 
you can see a live benchmark [here](https://github.com/Hexilee/async-postgres/actions?query=workflow%3ABenchmark).

### Develop

Run tests need a running postgres server and environment variables:
- `TCP_URL="postgresql:///<db>?host=<tcp host>&port=<port>&user=<user>&password=<passwd>"`
- `UDS_URL="postgresql:///<db>?host=<postgres uds dir>&port=<port>&user=<user>&password=<passwd>"`
