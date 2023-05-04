use std::io::{stdin, stdout};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

use termion::input::{TermRead};
use termion::event::{Event, Key};
use termion::raw::IntoRawMode;


fn play(rx: Arc<Mutex<Receiver<Event>>>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let source = SineWave::new(440.0).amplify(0.10);
    println!("Playing sound...\r");

    sink.append(source);
    thread::Builder::new().name("event_listener".to_string()).spawn(move || 'sound: loop {
        match rx.lock().unwrap().recv() {
            Ok(v) => match v {
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
                    println!("Done playing.\r");
                    break 'sound;
                },
                _ => println!("Key not supported\r"),
            },
            Err(e) => {
                println!("Error: {}", e);
                break 'sound;
            },
        }
    }).expect("Listener thread should be created").join().unwrap();
}

fn input_thread(tx: Sender<Event>) -> thread::JoinHandle<()> {
    std::thread::Builder::new().name("input_thread".to_string()).spawn(move || loop {
        let cmd = get_input();
        println!("Input: {:?}\r", cmd);
        // todo: spawn new channel on each loop
        let tx1 = mpsc::Sender::clone(&tx);
        match tx1.send(cmd) {
            Ok(_) => {},  // Do nothing, cmd is successfully sent
            Err(e) => {
                println!("{}, quitting...\r", e);
                break;
            }
        };
    }).expect("Input thread should be created.")
}

fn get_input() -> Event {
    let stdin = stdin();
    // Immediately returns input characters, i.e. no need for pressing Enter.
    // Only works if you do `let mut <variable>`. 
    let mut stdout = stdout().into_raw_mode().unwrap();

    println!("Captures Mouse Key events.\r");
    println!("Press p to play sound.\r");
    println!("Press q to quit.\r");

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
