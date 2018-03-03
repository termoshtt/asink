extern crate asink;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use asink::*;
use std::sync::mpsc::Sender;

#[derive(Serialize)]
struct Doc {
    id: usize,
    data: Vec<f64>,
}

fn experiment(s: Sender<Doc>) {
    for i in 0..5 {
        let doc = Doc {
            id: i,
            data: vec![i as f64],
        };
        s.send(doc).unwrap();
    }
}

#[test]
fn stdout() {
    let sink = json::StdoutSink::new();
    let (s, th) = sink.run();
    experiment(s);
    th.join().unwrap();
}

#[test]
fn json() {
    let sink = json::JsonSink::from_str("test.json");
    let (s, th) = sink.run();
    experiment(s);
    th.join().unwrap();
}

#[test]
fn msgpack() {
    let sink = msgpack::MsgpackSink::from_str("test.msg");
    let (s, th) = sink.run();
    experiment(s);
    th.join().unwrap();
}
