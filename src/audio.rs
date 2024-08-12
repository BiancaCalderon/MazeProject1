use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

pub struct AudioPlayer {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    audio_file: String, // Guardar el archivo de audio para reiniciarlo si es necesario
}

impl AudioPlayer {
    pub fn new(music_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let file = BufReader::new(File::open(music_file)?);
        let source = Decoder::new(file)?;
        sink.append(source);
        sink.pause();

        Ok(AudioPlayer {
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
            audio_file: music_file.to_string(),
        })
    }

    pub fn play(&self) {
        if let Ok(mut sink) = self.sink.lock() {
            if sink.empty() { // Verifica si el audio terminó
                self.restart();
            }
            sink.play();
        } else {
            eprintln!("Failed to lock the sink for playback.");
        }
    }

    pub fn pause(&self) {
        if let Ok(mut sink) = self.sink.lock() {
            sink.pause();
        } else {
            eprintln!("Failed to lock the sink to stop playback.");
        }
    }

    pub fn set_volume(&self, volume: f32) {
        if let Ok(mut sink) = self.sink.lock() {
            sink.set_volume(volume);
        } else {
            eprintln!("Failed to lock the sink to set volume.");
        }
    }

    fn restart(&self) {
        if let Ok(mut sink) = self.sink.lock() {
            sink.stop(); // Detén el audio actual
            let file = BufReader::new(File::open(&self.audio_file).unwrap());
            let source = Decoder::new(file).unwrap().repeat_infinite(); // Repite el audio infinitamente
            sink.append(source);
        }
    }
}
