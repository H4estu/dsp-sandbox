use fundsp::hacker::*;


fn create_oscillator(f: f64, m: f64, normalize: Option<bool>) -> Wave64 {
    let mut node = sine_hz(f) * f * m + f >> sine();
    let mut wave = Wave64::render(44100.0, 3.0, &mut node);

    if normalize.unwrap() {
        wave.normalize()
    }

    wave
}

fn main() {
    println!("Exploring the 🌊's...");

    let f = 440.0;
    let m = 0.0;  // Modulation amplitude. Increasing this increases the brightness of the tone.

    create_oscillator(f, m, Some(true))
    .save_wav16("test/sounds/fm_oscillator.wav")
    .expect("Could not save file.")
}