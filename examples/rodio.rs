use std::io::{stdin, stdout, Write};
use std::time::Duration;
use std::thread;

use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

use termion::input::{MouseTerminal, TermRead};
use termion::event::{Event, Key, MouseEvent};
use termion::raw::IntoRawMode;


fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let source = SineWave::new(440.0).amplify(0.10);
    // sink.append(source);

    // sink.sleep_until_end();

    // for _ in 0..5 {
    //     thread::sleep(Duration::from_secs(1));  // Start sound on initial loop
    //     sink.pause();  // pause after 1 second.

    //     thread::sleep(Duration::from_secs(1));
    //     sink.play();
    // }

    // let mut buffer = String::new();
    // let input = match io::stdin().read_line(&mut buffer) {
    //     Ok(n) => {
    //         println!("Bytes read: {}", n);
    //         println!("User entered: {}", buffer);
    //     },
    //     Err(e) => {
    //         println!("Cannae read: {e}.")
    //     }
    // };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    // write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
    println!("Captures Mouse Key events\r\nPress q to quit.\r");

    for c in stdin.events() {

        let event = c.unwrap();
        match event {
            Event::Key(Key::Char('q')) => break,
            // Event::Key(Key::Char(c)) => c,
            Event::Mouse(_) => todo!("Mouse events"),
            Event::Key(Key::Left) => todo!("Left arrow key"),
            // Event::Unsupported(e) =>  println!("Error: {:?}", e),
            _ => println!("Unsupported event\r"),
        }
        // Immediately returns input characters, i.e. no need for pressing Enter.
        stdout.flush().unwrap();
    }

}
