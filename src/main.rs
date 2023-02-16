use fundsp::hacker::*;

enum NoiseColor {
    Brown,
    Pink,
    White,
}

fn render_noise(noise: NoiseColor) {
    match noise {
        NoiseColor::Brown => println!("Brown Noise"),
        NoiseColor::Pink => println!("Pink Noise"),
        NoiseColor::White => println!("White Noise"),
    }
}

fn main() {
    println!("Hello, world!");

    render_noise(NoiseColor::Brown);
    render_noise(NoiseColor::Pink);
    render_noise(NoiseColor::White);

    let pink_noise = Wave64::render(44100.0, 3.0, &mut (pink()));
    let mut pink_filtered = pink_noise.filter(
        3.0,
        &mut ((pass() | lfo(|t| (xerp11(110.0, 880.0, spline_noise(0, t * 5.0)), 1.0)))
            >> bandpass()),
    );
    pink_filtered.normalize();

    let brown_noise = Wave64::render(44100.0, 3.0, &mut (brown()));
    let mut brown_filtered = brown_noise.filter(
        3.0,
        &mut ((pass() | lfo(|t| (xerp11(110.0, 880.0, spline_noise(0, t * 5.0)), 1.0)))
            >> bandpass()),
    );
    brown_filtered.normalize();

    let white_noise = Wave64::render(44100.0, 3.0, &mut (white()));
    let mut white_filtered = white_noise.filter(
        3.0,
        &mut ((pass() | lfo(|t| (xerp11(110.0, 880.0, spline_noise(0, t * 5.0)), 1.0)))
            >> bandpass()),
    );
    white_filtered.normalize();

    pink_filtered
        .save_wav16("test/sounds/pink_noise_filtered.wav")
        .expect("Could not save.");

    brown_filtered
        .save_wav16("test/sounds/brown_noise_filtered.wav")
        .expect("Could not save");

    white_filtered
        .save_wav16("test/sounds/white_noise_filtered.wav")
        .expect("Could not save");
}
