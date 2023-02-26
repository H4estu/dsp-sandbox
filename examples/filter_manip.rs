use std::sync::Arc;

use fundsp::hacker::*;


fn main() {
    println!("Exploring the 🌊's...");

    let SAMPLE_RATE = 44100.0;  // [Hz, s-1]
    let DURATION = 3.0;  // [s]

    let mut node = (pass() | lfo(|t| (xerp11(110.0, 880.0, spline_noise(0, t*5.0)), 10.))) >> bandpass();

    let wave = Arc::new(Wave64::render(SAMPLE_RATE, DURATION, &mut node));
    // let wave_player = Wave64Player::new(&wave, 0, 0, 1, Some(0));
}