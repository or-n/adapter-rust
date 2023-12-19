use rodio::{OutputStream, Sink};
use rodio::buffer::SamplesBuffer;

pub fn play(rate: u32, data: &[f32]) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let buffer = |xs| SamplesBuffer::new(1, rate, xs);
    sink.append(buffer(data));
    sink.sleep_until_end();
}