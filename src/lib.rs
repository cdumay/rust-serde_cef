// Copyright 2019-present, OVH SAS
// All rights reserved.
//
// This OVH Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.
//

//! CEF is an extensible, text-based format designed to support multiple device types by offerring the
//! most relevant information. Message syntaxes are reduced to work with ESM normalization.
//! Specifically, CEF defines a syntax for log records comprised of a standard header and a variable
//! extension, formatted as key-value pairs.
//!
//! ```
//! Sep 19 08:26:10 host CEF:0|Security|threatmanager|1.0|100|worm successfully stopped|10|src=10.0.0.1 dst=2.1.2.2 spt=1232
//! ```
//!
//! # Quickstart
//!
//! You can start using it by first adding it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! serde_derive = "1.0"
//! serde_cef = "0.1"
//! ```
//!
//! Then, create a structure which implement `serde::Serialize` / `serde::Deserialize` traits and
//! use the structure as extention in the `serde_cef::CefRecord`.
//!
//! ```rust
//! extern crate serde_cef;
//! #[macro_use]
//! extern crate serde_derive;
//! 
//! use serde_cef::{CefRecord, CefSeverity, CefSignatureId};
//!
//! #[derive(Serialize, Deserialize, Clone, Debug)]
//! struct Foo {
//!     a: String,
//!     b: u64,
//! }
//! 
//! fn main() {
//!     let rec = CefRecord {
//!         headers: None,
//!         version: 0,
//!         device_vendor: "Fake".to_string(),
//!         device_product: "Product".to_string(),
//!         device_version: "0.1".to_string(),
//!         signature_id: CefSignatureId::U64(0),
//!         signature: "Nothing".to_string(),
//!         severity: CefSeverity::U8(6),
//!         extensions: Foo { a: "subtest".into(), b: 695217 },
//!     };
//!     let as_string = serde_cef::to_string(&rec).unwrap();
//!     println!("{}", &as_string);
//!     println!("{:?}", serde_cef::from_str::<Foo>(&as_string).unwrap());
//! }
//! ```
//! **Output**:
//! ```text
//! CEF:0|Fake|Product|0.1|0|Nothing|6|a=subtest b=695217
//! CefRecord { headers: None, version: 0, device_vendor: "Fake", device_product: "Product", device_version: "0.1", signature_id: U64(0), signature: "Nothing", severity: U8(6), extensions: Foo { a: "subtest", b: 695217 } }
//! ```
//!
//!
//! ## Feature ovh-ldp
//!
//! The feature `ovh-ldp` allow to suffix fields names to suits to the [LDP naming conventions](https://docs.ovh.com/fr/logs-data-platform/field-naming-conventions/).
//!
//! In your `Cargo.toml`, set:
//!
//! ```toml
//! [dependencies]
//! serde_value_flatten = { version = "0.1", features = ["ovh-ldp"] }
//! ```
//!
//! Re-run the previous example, and now the output will be :
//!
//! ```text
//! CEF:0|Fake|Product|0.1|0|Nothing|6|a:subtest b_double:695217
//! ```

#![doc(
html_logo_url = "https://eu.api.ovh.com/images/com-square-bichro.png",
html_favicon_url = "https://www.ovh.com/favicon.ico",
)]
//#![deny(warnings, missing_docs)]
#![feature(try_trait)]
extern crate chrono;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_value;
extern crate serde_value_flatten;

pub use de::from_str;
pub use record::{CefRecord, CefSeverity, CefSignatureId};
pub use result::{CefError, CefResult};
pub use ser::to_string;
pub use utils::{extract_hostname_from_headers, extract_ts_from_headers, parse_ts};

mod result;
mod de;
mod ser;
mod record;
mod utils;

