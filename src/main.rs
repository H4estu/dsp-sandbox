use anyhow;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use dasp::{signal, Sample, Signal};

use iced::widget::{button, column, horizontal_rule, text};
use iced::{Alignment, Application, executor, Command, Element, Settings, Theme};

use std::sync::mpsc;
use std::time::Duration;


#[derive(Debug, Clone, Copy)]
enum Message {
    TogglePlayback,
}

// #[derive(Default)]
struct Waveform {
    is_playing: bool,
    _host: cpal::Host,
    device: cpal::Device,
    supported_stream_config: cpal::SupportedStreamConfig,

}

impl Default for Waveform {

    fn default() -> Self {
        // Setup default audio backend
        let (_host, device, config) = host_device_setup().unwrap();
        Waveform {
            is_playing: false,
            _host: _host,
            device: device,
            supported_stream_config: config,
        }
    }
}


impl Application for Waveform {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Waveform, Command<Self::Message>) {
        (Waveform::default(), Command::none())
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::Dark
    }

    fn title(&self) -> String {
        String::from("🌊")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TogglePlayback => self.is_playing = !self.is_playing
        };
        if self.is_playing {
            play_sound_cpal();
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            text("Toggle Play"),
            horizontal_rule(10),
            button(if self.is_playing{"pause"} else {"play"}).on_press(Message::TogglePlayback),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}

fn play_sound_cpal() {

    let config = Waveform::default().supported_stream_config;
    let device = Waveform::default().device;

    // Only care about f32 for now.
    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        _ => panic!("Not implemented for non-f32"),
    }.unwrap();

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
    let one_sec = (config.sample_rate.0) as usize;
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
