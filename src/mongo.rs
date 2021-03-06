//! Sink with MongoDB backend

use serde::Serialize;
use std::thread::{spawn, JoinHandle};
use std::sync::mpsc::{channel, Sender};

use bson;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;

use super::Sink;

/// Sink with MongoDB backend
///
/// Each data send to this sink will be serialized into `bson::Document`
/// i.e. the data must be serialized into `Document`.
#[derive(new, Debug, Clone)]
pub struct MongoSink {
    host: String,
    port: u16,
    db: String,
    collection: String,
}

impl MongoSink {
    /// Connect to mongodb with default setting `localhost:27017`
    pub fn local(db: &str, collection: &str) -> Self {
        Self::new(
            "localhost".to_string(),
            27017,
            db.to_string(),
            collection.to_string(),
        )
    }
}

fn into_bson_document<T: Serialize>(val: T) -> bson::Document {
    use bson::Bson::*;
    match bson::to_bson(&val).unwrap() {
        Document(d) => d,
        _ => panic!("Input data must be converted into BSON::Document"),
    }
}

impl<Doc: 'static + Send + Serialize> Sink<Doc> for MongoSink {
    fn run(self) -> (Sender<Doc>, JoinHandle<()>) {
        let (s, r) = channel::<Doc>();
        let th = spawn(move || {
            let cli = ::mongodb::Client::connect(&self.host, self.port)
                .expect("Unable to connect to MongoDB");
            let coll = cli.db(&self.db).collection(&self.collection);
            loop {
                match r.recv() {
                    Ok(doc) => {
                        coll.insert_one(into_bson_document(doc), None)
                            .expect("Failed to insert document");
                    }
                    Err(_) => break,
                }
            }
        });
        (s, th)
    }
}
