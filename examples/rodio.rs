use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
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


fn play(rx: Arc<Mutex<Receiver<Event>>>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let source = SineWave::new(440.0).amplify(0.10);
    println!("Playing sound...\r");

    sink.append(source);
    thread::spawn(move || 'sound: loop {
        match rx.lock().unwrap().recv().unwrap() {
            Event::Key(Key::Char('p')) => {
                println!("Paused\r");
                sink.pause();
            },
            Event::Key(Key::Char('r')) => {
                println!("Resuming\r");
                sink.play();
            },
            Event::Key(Key::Char('s')) => {
                println!("Stopping\r");
                sink.stop();
                break 'sound;
            },
            _ => println!("Key not supported\r"),
        }
    }).join().unwrap();

    println!("Done playing.\r");
}

fn input_thread(tx: Sender<Event>) -> thread::JoinHandle<()> {
    std::thread::Builder::new().name("input_thread".to_string()).spawn(move || loop {
        let cmd = get_input();
        println!("Input: {:?}", cmd);
        let tx1 = mpsc::Sender::clone(&tx);
        tx1.send(cmd).expect("Input thread should be able to capture multiple events.");
    }).expect("Input thread should be able to be created.")

}

fn get_input() -> Event {
    let stdin = stdin();
    // Immediately returns input characters, i.e. no need for pressing Enter.
    // Only works if you do `let mut <variable>`. 
    let mut stdout = stdout().into_raw_mode().unwrap();
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    // write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    // stdout.flush().unwrap();
    println!("Captures Mouse Key events.\r");
    println!("Press p to play sound.\r");
    println!("Press q to quit.\r");

    // for c in stdin.events() {
    //     let event = c.unwrap();
    //     match event {
    //         Event::Key(Key::Char('q') | Key::Ctrl('c')) => break,
    //         Event::Key(Key::Char('p')) => play(),
    //         // Event::Key(Key::Char(c)) => c,
    //         Event::Mouse(_) => todo!("Mouse events"),
    //         // Event::Unsupported(e) =>  println!("Error: {:?}", e),
    //         _ => println!("{:?}\r", event),
    //     }
    //     // stdout.flush().unwrap();
    // }
    match stdin.events().next() {
        Some(n) => n.unwrap(),
        None => Event::Key(Key::Char('q'))
    }
}

fn play_thread(rx: Arc<Mutex<Receiver<Event>>>) {
    let player = std::thread::Builder::new().name("play_thread".to_string()).spawn(move || {
        let cmd = rx.lock().unwrap().recv().unwrap();
        match cmd {
            Event::Key(Key::Char('p')) => play(Arc::clone(&rx)),
            Event::Key(Key::Char('q') | Key::Ctrl('c')) => std::process::exit(0),
            _ => println!("{:?}\r", cmd),
        }
    }).unwrap();

    player.join().expect("Player thread should play the sound.");
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let input_thread = input_thread(tx);

    let receiver = Arc::new(Mutex::new(rx));
    play_thread(receiver);

    input_thread.join().unwrap();
}
