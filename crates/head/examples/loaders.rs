#[cfg(not(target_arch = "wasm32"))]
mod async_loader {
    pub use bytes::Bytes;
    pub use rrr_core::chart::NoteColor;
    pub use std::{io::Cursor, pin::Pin};
    pub use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

    pub struct NoteskinList(Vec<NoteskinMetadata>);

    pub struct NoteskinMetadata {
        name: String,
        author: String,
        data: Noteskin,
    }

    pub struct Noteskin {
        note_width: usize,
        note_height: usize,
        color_indexs: Vec<NoteColor>,
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
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use async_loader::*;

    let test_bytes: Bytes =
        Bytes::from_static(include_bytes!("../../../data/default_noteskin.png"));

    let chart_data = NoteskinLoading::from_bytes(test_bytes);

    let mut reader = BufReader::new(chart_data);
    let mut buffer = Vec::new();
    let debug = reader.read_to_end(&mut buffer).await.unwrap();

    log::info!("size: {:?}", debug);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    unimplemented!() // `Loader example is not yet implemented for wasm.`
}
