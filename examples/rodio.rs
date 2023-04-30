use std::io::{stdin, stdout, Write};
use std::time::Duration;
use std::thread;

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{Pausable, PeriodicAccess, SineWave, Source};

use termion::input::{MouseTerminal, TermRead};
use termion::event::{Event, Key, MouseEvent};
use termion::raw::IntoRawMode;

fn set_paused(source: SineWave) {
    println!("Pausing sound...");
    source.pausable(false).set_paused(true)
}


fn play() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let source = SineWave::new(440.0).amplify(0.10);
    println!("Playing sound...\r");

    sink.append(source);
    thread::spawn(move || {
        sink.sleep_until_end();
        
    });
    thread::sleep(Duration::from_secs(2));

    // sink.sleep_until_end();  // put this in a thread, have the thread get killed by a listener when an event is sent?


    println!("Done playing.\r");
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    // write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
    println!("Captures Mouse Key events.\r");
    println!("Press p to play sound.\r");
    println!("Press q to quit.\r");

    for c in stdin.events() {

        let event = c.unwrap();
        match event {
            Event::Key(Key::Char('q') | Key::Ctrl('c')) => break,
            Event::Key(Key::Char('p')) => play(),
            // Event::Key(Key::Char(c)) => c,
            Event::Mouse(_) => todo!("Mouse events"),
            // Event::Unsupported(e) =>  println!("Error: {:?}", e),
            _ => println!("{:?}\r", event),
        }
        // Immediately returns input characters, i.e. no need for pressing Enter.
        stdout.flush().unwrap();
    }
}
