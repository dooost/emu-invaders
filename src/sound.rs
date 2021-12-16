use std::{collections::HashMap, io::Cursor};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub struct SoundController {
    // This should ideally be an array, but collecting into arrays doesn't work which makes it a bit uncomfortable
    wavs: Vec<Vec<u8>>,
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    repeating_sounds: HashMap<usize, Sink>,
}

impl SoundController {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let wavs = (0..10)
            .map(|i| {
                let wav_path = format!("res/sound/{}.wav", i);
                std::fs::read(wav_path).expect("Missing sound")
            })
            .collect();

        Self {
            wavs,
            stream_handle,
            _stream,
            repeating_sounds: HashMap::new(),
        }
    }

    pub fn play_once(&self, i: usize) {
        let buf = self.wavs[i].clone();
        let cursor = Cursor::new(buf);
        let source = Decoder::new_wav(cursor).unwrap();
        self.stream_handle
            .play_raw(source.convert_samples())
            .unwrap();
    }

    pub fn play_repeating(&mut self, i: usize) {
        if self.repeating_sounds.contains_key(&i) {
            return;
        }

        let buf = self.wavs[i].clone();
        let cursor = Cursor::new(buf);
        let source = Decoder::new_wav(cursor).unwrap().repeat_infinite();
        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);

        self.repeating_sounds.insert(i, sink);
    }

    pub fn stop_repeating(&mut self, i: usize) {
        match self.repeating_sounds.get(&i) {
            Some(sink) => {
                sink.stop();
                self.repeating_sounds.remove(&i);
            }
            None => return,
        }
    }
}
