use serde::Serialize;
use std::sync::mpsc::{channel, Sender};
use std::thread::{spawn, JoinHandle};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::fs::File;

use super::Sink;

#[derive(Debug, Clone)]
pub struct MsgpackSink {
    filename: PathBuf,
}

impl MsgpackSink {
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

impl<Doc: 'static + Send + Serialize> Sink<Doc> for MsgpackSink {
    fn run(self) -> (Sender<Doc>, JoinHandle<()>) {
        let (s, r) = channel::<Doc>();
        let th = spawn(move || {
            let mut buf = BufWriter::new(File::create(self.filename).ok().unwrap());
            let mut enc = ::rmp_serde::Serializer::new(&mut buf);
            loop {
                match r.recv() {
                    Ok(doc) => doc.serialize(&mut enc).unwrap(),
                    Err(_) => break,
                }
            }
        });
        (s, th)
    }
}
