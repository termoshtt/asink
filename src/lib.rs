//! Async sink of time-series data

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
