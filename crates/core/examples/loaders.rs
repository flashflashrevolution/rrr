// Loader for bytes
// Loader for URL / LocalStorage Cache
// Loader for file system

use std::{io::Cursor, pin::Pin};

use bytes::Bytes;
use rrr_core::note::Color;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

// Empty Noteskin List
// Download the noteskin list data from the server.
// (optional) Prefetch the noteskin images into the localstorage.
// Load each noteskin list on demand.

pub struct NoteskinList(Vec<NoteskinMetadata>);

pub struct NoteskinMetadata {
    name: String,
    author: String,
    data: Noteskin,
}

pub struct Noteskin {
    note_width: usize,
    note_height: usize,
    color_indexs: Vec<Color>,
    rotations: Vec<usize>,
    rows: usize,
}

pub struct NoteskinLoading {
    inner: LoaderType,
}

pub struct NoteskinLoaded {}

enum LoaderType {
    Bytes(Cursor<Bytes>),
    AsyncRead(Pin<Box<dyn AsyncRead + Send>>),
}

impl NoteskinLoading {
    pub fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        Self {
            inner: LoaderType::Bytes(Cursor::new(bytes.into())),
        }
    }
}

impl AsyncRead for NoteskinLoading {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match &mut self.inner {
            LoaderType::Bytes(cursor) => AsyncRead::poll_read(Pin::new(cursor), cx, buf),
            LoaderType::AsyncRead(_) => todo!(),
        }
    }
}

#[tokio::main]
async fn main() {
    let test_bytes: Bytes =
        Bytes::from_static(include_bytes!("../../../data/default_noteskin.png"));

    let chart_data = NoteskinLoading::from_bytes(test_bytes);

    let mut reader = BufReader::new(chart_data);
    let mut buffer = Vec::new();
    let debug = reader.read_to_end(&mut buffer).await.unwrap();

    log::info!("size: {:?}", debug);
}
