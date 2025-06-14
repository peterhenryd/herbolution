use rodio::{OutputStream, OutputStreamHandle};

pub struct Audio {
    _stream: OutputStream,
    _handle: OutputStreamHandle,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();

        Self { _stream: stream, _handle: handle }
    }
}
