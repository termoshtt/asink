use serde::Serialize;
use serde_json::to_writer;
use std::sync::mpsc::{channel, Sender};
use std::thread::{spawn, JoinHandle};
use std::io::{stdout, BufWriter};
use std::path::{Path, PathBuf};
use std::fs::File;

use super::Sink;

/// Output data as [JSON lines] (a.k.a [Newline-deliminated JSON]) like following:
///
/// ```
/// {"a": 1, "b": [1, 2, 3]}
/// {"a": 2, "b": [2, 2, 3]}
/// ```
///
/// [JSON lines]: http://jsonlines.org/
/// [Newline-deliminated JSON]: http://ndjson.org/
#[derive(Debug, Clone)]
pub struct JsonSink {
    filename: PathBuf,
}

impl JsonSink {
    pub fn new(path: &Path) -> Self {
        Self {
            filename: PathBuf::from(path),
        }
    }
}

impl<Doc: 'static + Send + Serialize> Sink<Doc> for JsonSink {
    fn run(self) -> (Sender<Doc>, JoinHandle<()>) {
        let (s, r) = channel::<Doc>();
        let th = spawn(move || {
            let mut buf = BufWriter::new(File::create(self.filename).ok().unwrap());
            loop {
                match r.recv() {
                    Ok(doc) => to_writer(&mut buf, &doc).unwrap(),
                    Err(_) => break,
                }
            }
        });
        (s, th)
    }
}

/// Output data as JSON into stdout
#[derive(Debug, Clone, new)]
pub struct StdoutSink {}

impl<Doc: 'static + Send + Serialize> Sink<Doc> for StdoutSink {
    fn run(self) -> (Sender<Doc>, JoinHandle<()>) {
        let (s, r) = channel::<Doc>();
        let th = spawn(move || {
            let out = stdout();
            let mut buf = BufWriter::new(out.lock());
            loop {
                match r.recv() {
                    Ok(doc) => to_writer(&mut buf, &doc).unwrap(),
                    Err(_) => break,
                }
            }
        });
        (s, th)
    }
}
