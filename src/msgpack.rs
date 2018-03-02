use serde::Serialize;
use std::sync::mpsc::{channel, Sender};
use std::thread::{spawn, JoinHandle};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::fs::File;

use super::Sink;

#[derive(Debug, Clone)]
pub struct MsgpackStorage {
    filename: PathBuf,
}

impl MsgpackStorage {
    pub fn new(path: &Path) -> Self {
        Self {
            filename: PathBuf::from(path),
        }
    }
}

impl<Doc: 'static + Send + Serialize> Sink<Doc> for MsgpackStorage {
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
