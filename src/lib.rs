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

/// Start a thread for saving time-series
pub trait Sink<Document: Send + Serialize> {
    fn run(self) -> (Sender<Document>, JoinHandle<()>);
}