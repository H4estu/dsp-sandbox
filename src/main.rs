use fundsp::hacker::*;


enum NoiseColor {
    Brown,
    Pink,
    White,
}


impl NoiseColor {
    fn render(&self) -> Wave64 {
        let wave = match self {
            Self::Brown => Wave64::render(44100.0, 3.0, &mut (brown())),
            Self::Pink => Wave64::render(44100.0, 3.0, &mut (pink())),
            Self::White => Wave64::render(44100.0, 3.0, &mut (white())),
        };

        return wave;
    }
}


fn main() {
    println!("Hello, world!");

    let color = NoiseColor::Brown;
    let mut node = (pass() | lfo(|t| (xerp11(110.0, 880.0, spline_noise(0, t*5.0)), 10.))) >> bandpass();

    color
        .render()
        .filter(3.0, &mut node)
        .save_wav16("test/sounds/brown_noise_filtered.wav")
        .expect("Could not save file.");

    let color = NoiseColor::Pink;
    color
        .render()
        .filter(3.0, &mut node)
        .save_wav16("test/sounds/pink_noise_filtered.wav")
        .expect("Could not save file.");

    let color = NoiseColor::White;
    color
        .render()
        .filter(3.0, &mut node)
        .save_wav16("test/sounds/white_noise_filtered.wav")
        .expect("Could not save file.");
}
