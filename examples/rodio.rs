use std::time::Duration;

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

use std::thread;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let source = SineWave::new(440.0).amplify(0.10);
    sink.append(source); 

    // sink.sleep_until_end();

    for _ in 0..5 {
        thread::sleep(Duration::from_secs(1));  // Start sound on initial loop
        sink.pause();  // pause after 1 second.

        thread::sleep(Duration::from_secs(1));
        sink.play();
    }
}
