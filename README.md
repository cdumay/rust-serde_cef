# serde_cef

[![Build Status](https://travis-ci.org/cdumay/rust-serde_cef.svg?branch=master)](https://travis-ci.org/cdumay/rust-serde_cef) 
[![Latest version](https://img.shields.io/crates/v/serde_cef.svg)](https://crates.io/crates/serde_cef)
[![Documentation](https://docs.rs/serde_cef/badge.svg)](https://docs.rs/serde_cef) 
![License](https://img.shields.io/crates/l/serde_cef.svg)

CEF is an extensible, text-based format designed to support multiple device types by offerring the
most relevant information. Message syntaxes are reduced to work with ESM normalization.
Specifically, CEF defines a syntax for log records comprised of a standard header and a variable
extension, formatted as key-value pairs.

```rust
Sep 19 08:26:10 host CEF:0|Security|threatmanager|1.0|100|worm successfully stopped|10|src=10.0.0.1 dst=2.1.2.2 spt=1232
```

## Quickstart

You can start using it by first adding it to your `Cargo.toml`:

```toml
[dependencies]
serde_derive = "1.0"
serde_cef = "0.1"
```

Then, create a structure which implement `serde::Serialize` / `serde::Deserialize` traits and
use the structure as extention in the `serde_cef::CefRecord`.

```rust
extern crate serde_cef;
#[macro_use]
extern crate serde_derive;

use serde_cef::{CefRecord, CefSeverity, CefSignatureId,to_string, from_str};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Foo {
    a: String,
    b: u64,
}

fn main() {
    let rec = CefRecord {
        headers: None,
        version: 0,
        device_vendor: "Fake".to_string(),
        device_product: "Product".to_string(),
        device_version: "0.1".to_string(),
        signature_id: CefSignatureId::U64(0),
        signature: "Nothing".to_string(),
        severity: CefSeverity::U8(6),
        extensions: Foo { a: "subtest".into(), b: 695217 },
    };
    let as_string = to_string(&rec).unwrap();
    println!("{}", &as_string);
    println!("{:?}", from_str::<Foo>(&as_string).unwrap());
}
```
**Output**:
```
CEF:0|Fake|Product|0.1|0|Nothing|6|a:subtest b:695217
CefRecord { headers: None, version: 0, device_vendor: "Fake", device_product: "Product", device_version: "0.1", signature_id: U64(0), signature: "Nothing", severity: U8(6), extensions: Foo { a: "subtest", b: 695217 } }
```


### Feature ovh-ldp

The feature `ovh-ldp` allow to suffix fields names to suits to the [LDP naming conventions](https://docs.ovh.com/fr/logs-data-platform/field-naming-conventions/).

In your `Cargo.toml`, set:

```toml
[dependencies]
serde_value_flatten = { version = "0.1", features = ["ovh-ldp"] }
```

Re-run the previous example, and now the output will be like :

```
CEF:0|Fake|Product|0.1|0|Nothing|6|a:subtest b_double:695217
```

License: BSD-3-Clause
