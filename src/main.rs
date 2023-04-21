use anyhow;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use dasp::{signal, Sample, Signal};
use signal::{Sine, ConstHz};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;


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
    let mut synth = hz
        .clone()
        .sine();

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

// fn write_data<T>(output: &mut [T], channels: usize, complete_tx: &mpsc::SyncSender<()>, signal: &mut dyn Iterator<Item = f32>) 
fn write_data<T>(output: &mut [T], channels: usize, complete_tx: &mpsc::SyncSender<()>, signal: &mut Sine<ConstHz>) 
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        // let sample = match signal.next() {
        //     None =>  {
        //         complete_tx.try_send(()).ok();
        //         0.0
        //     },
        //     Some(sample) => sample,
        // };
        let sample = signal.next() as f32;

        let value: T = cpal::Sample::from_sample::<f32>(sample);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn play_signal(device: cpal::Device, config: cpal::SupportedStreamConfig) {
    // Only care about f32 for now.
    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        _ => panic!("Not implemented for non-f32"),
    }.unwrap();

}

fn main() {
    println!("Exploring the 🌊's...");

    thread::spawn(|| {
        println!("Playing signal in thread");
        let (_host, device, ssg) = host_device_setup().unwrap();
        play_signal(device, ssg);
        thread::sleep(Duration::from_millis(1));
    }).join().unwrap();

    // play_signal(device, ssg);
}
