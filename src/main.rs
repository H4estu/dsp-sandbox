use fundsp::hacker::*;

fn main() {
    println!("Hello, world!");

    let wave1 = Wave64::render(44100.0, 3.0, &mut (pink()));
    let mut wave2 = wave1.filter(3.0, &mut ((pass() | lfo(|t| (xerp11(110.0, 880.0, spline_noise(0, t * 5.0)), 1.0))) >> bandpass()));
    // wave2.normalize();
    
    wave2.save_wav16("test/sounds/pink_noise_filtered.wav").expect("Could not save.");
}
