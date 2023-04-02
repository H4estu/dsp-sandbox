use anyhow;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use dasp::{signal, Sample, Signal};

// use fundsp::hacker::*;

use iced::widget::{button, column, horizontal_rule, horizontal_space, row, text};
use iced::widget::canvas;
// use iced::widget::canvas::stroke::{self, Stroke};
use iced::{Color, executor, Renderer};
use iced::{Alignment, Application, Command, Element, Length, Point, Rectangle, Settings, Subscription, Theme};
use iced::{window};

use std::process::Command as ShellCommand;
use std::sync::mpsc;
use std::time::{Duration, Instant};


#[derive(Debug, Clone, Copy)]
enum Message {
    // IncreaseFrequency,
    // IncreaseAmplitude,
    // DecreaseFrequency,
    // DecreaseAmplitude,
    // Tick(Instant)
    TogglePlayback,
}

#[derive(Default)]
struct Waveform {
    is_playing: bool,
}

// #[derive(Debug)]
// struct Waveform {
//     state: State
// }

// #[derive(Debug)]
// struct State {
//     frequency: f32,
//     amplitude: f32,
//     cache: canvas::Cache,
//     now: Instant
// }

// impl State {
//     pub fn new() -> State {
//         let (width, height) = window::Settings::default().size;
//         let now = Instant::now();

//         State {
//             frequency: 44100.0,
//             amplitude: 1.0,
//             cache: Default::default(),
//             now
//         }
//     }

//     pub fn update(&mut self, now: Instant) {
//         self.now = now;
//         self.cache.clear();
//     }
// }


// impl<Message> canvas::Program<Message> for State {
//     type State = ();

//     fn draw(
//         &self,
//         _state: &Self::State,
//         _theme: &Theme,
//         bounds: Rectangle,
//         _cursor: canvas::Cursor
//     ) -> Vec<canvas::Geometry> {

//         let waveform = self.cache.draw(bounds.size(), |frame| {

//             let line = canvas::Path::line(Point::ORIGIN, Point::new(100., 100.));
//             frame.translate(frame.center() - Point::ORIGIN);
//             frame.fill(&line, Color::BLACK);
//             frame.stroke(&line, Stroke{
//                 style: stroke::Style::Solid(Color::from_rgb(255., 0., 0.)),
//                 width: 2.0,
//                 ..Stroke::default()
//             })
//         });

//         vec![waveform]
//     }
// }



impl Application for Waveform {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Waveform, Command<Self::Message>) {
        (
            // Waveform {
            //     state: State::new()
            // },
            Waveform {
                is_playing: false,
            },
            Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dark
    }

    fn title(&self) -> String {
        String::from("🌊")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // Message::IncreaseFrequency => self.state.frequency += 10.0,
            // Message::DecreaseFrequency => self.state.frequency -= 10.0,
            // Message::IncreaseAmplitude => self.state.amplitude += 1.0,
            // Message::DecreaseAmplitude => self.state.amplitude -= 1.0
            // Message::Tick(instant) => self.state.update(instant),
            Message::TogglePlayback => {
                self.is_playing = !self.is_playing;

                if self.is_playing {
                    // play_sound_sox()
                    play_sound_cpal()
                }
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // canvas::Canvas::new(&self.state)
            // .width(Length::Fill)
            // .height(Length::Fill)
            // .into()
        column![
            text("Toggle Play"),
            horizontal_rule(10),
            button(if self.is_playing{"play"} else {"pause"}).on_press(Message::TogglePlayback),
        ]
        .align_items(Alignment::Center)
        .into()
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     window::frames().map(Message::Tick)
    // }
}

fn play_sound_cpal() {
    let (_host, device, config) = host_device_setup().unwrap();

    // Only care about f32 for now.
    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        _ => panic!("Not implemented for non-f32"),
    }.unwrap();

}

fn play_sound_sox() {
    println!("Playing a sweet 7th");
    let sound = ShellCommand::new("play")
        .arg("-n")
        .arg("-c 1")
        .arg("synth")
        // .arg("sin %-12")
        // .arg("sin %-9")
        // .arg("sin %-5")
        // .arg("sin %-2")
        .output()
        .expect("Could not play sound.");
    println!("{:?}", sound.stdout);
    println!("{:?}", sound.stderr);
}

fn host_device_setup() -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    println!("Setting up host deivce...");

    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device not available."))?;
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config: {:?}", config);

    Ok((host, device, config))
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error> 
where 
    T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32>,
{
    // Signal chain to play back one second of each oscillator at A4
    let hz = signal::rate(config.sample_rate.0 as f64).const_hz(440.);
    let one_sec = config.sample_rate.0 as usize;
    let mut synth = hz
        .clone()
        .sine()
        .take(one_sec)
        .chain(hz.clone().saw().take(one_sec))
        .chain(hz.clone().square().take(one_sec))
        .chain(hz.clone().noise_simplex().take(one_sec))
        .chain(signal::noise(0).take(one_sec))
        .map(|s| s.to_sample::<f32>() *0.2);

    // A channel for indicating when playback has completed
    let (complete_tx, complete_rx) = mpsc::sync_channel(1);

    // Create and run the stream
    let err_fn = |err| eprintln!("An error occured on the stream: {}", err);
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &complete_tx, &mut synth)
        },
        err_fn,
        Some(Duration::new(5, 0))
    )?;

    stream.play()?;

    // wait for playback to complete. Sounds won't play without these.
    complete_rx.recv().unwrap();
    stream.pause()?;

    Ok(())

}


fn write_data<T>(output: &mut [T], channels: usize, complete_tx: &mpsc::SyncSender<()>, signal: &mut dyn Iterator<Item = f32>) 
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let sample = match signal.next() {
            None =>  {
                complete_tx.try_send(()).ok();
                0.0
            },
            Some(sample) => sample,
        };
        let value: T = cpal::Sample::from_sample::<f32>(sample);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn main() -> iced::Result {
    println!("Exploring the 🌊's...");

    Waveform::run(Settings::default())    
}
