//! Async sink of time-series data based on serde-rs
//!
//! Example
//! --------
//!
//! ```rust
//! extern crate asink;
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use asink::*;
//! use std::sync::mpsc::Sender;
//!
//! /// This will be serialized into msgpack
//! #[derive(Serialize)]
//! struct Doc {
//!     id: usize,
//!     data: Vec<f64>,
//! }
//!
//! fn experiment(s: Sender<Doc>) {
//!     for i in 0..5 {
//!         let doc = Doc {
//!             id: i,
//!             data: vec![i as f64],
//!         };
//!         s.send(doc).unwrap(); // Send data to sink
//!     }
//! }
//!
//! fn main() {
//!     let sink = msgpack::MsgpackSink::from_str("test.msg");
//!     let (s, th) = sink.run(); // Sink start thread to write recieved data into msgpack
//!     experiment(s);
//!     th.join().unwrap();
//! }
//! ```

extern crate bson;
#[macro_use]
extern crate derive_new;
extern crate mongodb;
extern crate rmp;
extern crate rmp_serde;
extern crate serde;
extern crate serde_json;

use serde::Serialize;
use std::thread::JoinHandle;
use std::sync::mpsc::Sender;

pub mod mongo;
pub mod msgpack;
pub mod json;

/// Sink trait for storing time-series data
pub trait Sink<Document: Send + Serialize> {
    /// Start a thread for sink
    fn run(self) -> (Sender<Document>, JoinHandle<()>);
}
