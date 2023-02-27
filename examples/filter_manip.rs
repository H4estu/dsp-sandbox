use std::sync::Arc;

use fundsp::hacker::*;


fn main() {
    println!("Exploring the 🌊's...");

    let sample_rate = 44100.0;  // [Hz, s-1]
    let duration = 3.0;  // [s]

    let mut node = (pass() | lfo(|t| (xerp11(110.0, 880.0, spline_noise(0, t*5.0)), 10.))) >> bandpass();

    let wave = Arc::new(Wave64::render(sample_rate, duration, &mut node));
    // let wave_player = Wave64Player::new(&wave, 0, 0, 1, Some(0));
}