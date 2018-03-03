use serde::Serialize;
use serde_json::to_writer;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle};
use std::io::{stdout, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::fs::File;

use super::Sink;

/// Output data as [JSON lines] (a.k.a [Newline-deliminated JSON]) like following:
///
/// ```ignore
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
    pub fn from_path(path: &Path) -> Self {
        Self {
            filename: PathBuf::from(path),
        }
    }

    pub fn from_str(path: &str) -> Self {
        Self {
            filename: From::from(path.to_string()),
        }
    }
}

impl<Doc: 'static + Send + Serialize> Sink<Doc> for JsonSink {
    fn run(self) -> (Sender<Doc>, JoinHandle<()>) {
        let (s, r) = channel::<Doc>();
        let th = spawn(move || {
            let buf = BufWriter::new(File::create(self.filename).ok().unwrap());
            output_json(buf, r)
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
            let buf = BufWriter::new(out.lock());
            output_json(buf, r)
        });
        (s, th)
    }
}

fn output_json<Buf, Doc>(mut buf: Buf, r: Receiver<Doc>)
where
    Buf: Write,
    Doc: Serialize,
{
    loop {
        match r.recv() {
            Ok(doc) => {
                to_writer(&mut buf, &doc).unwrap();
                writeln!(buf, "").unwrap();
            }
            Err(_) => return,
        }
    }
}
