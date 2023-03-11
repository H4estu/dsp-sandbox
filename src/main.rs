use fundsp::hacker::*;
use iced::widget::{button, column, horizontal_rule, horizontal_space, row, text};
use iced::widget::canvas;
use iced::widget::canvas::stroke::{self, Stroke};
use iced::{Color, executor, Renderer};
use iced::{Alignment, Application, Command, Element, Length, Point, Rectangle, Settings, Subscription, Theme};
use iced::{window};

use std::time::Instant;


#[derive(Debug, Clone, Copy)]
enum Message {
    // IncreaseFrequency,
    // IncreaseAmplitude,
    // DecreaseFrequency,
    // DecreaseAmplitude,
    Tick(Instant)
}

#[derive(Debug)]
struct Waveform {
    state: State
}

#[derive(Debug)]
struct State {
    frequency: f32,
    amplitude: f32,
    cache: canvas::Cache,
    now: Instant
}

impl State {
    pub fn new() -> State {
        let (width, height) = window::Settings::default().size;
        let now = Instant::now();

        State {
            frequency: 44100.0,
            amplitude: 1.0,
            cache: Default::default(),
            now
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        self.cache.clear();
    }
}


impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: canvas::Cursor
    ) -> Vec<canvas::Geometry> {

        let waveform = self.cache.draw(bounds.size(), |frame| {
            let line = canvas::Path::line(Point::ORIGIN, Point::new(100., 100.));
            // frame.translate(frame.center() - Point::ORIGIN);
            frame.fill(&line, Color::BLACK);
            frame.stroke(&line, Stroke{
                style: stroke::Style::Solid(Color::from_rgb(255., 0., 0.)),
                width: 2.0,
                ..Stroke::default()
            })
        });

        vec![waveform]
    }
}



impl Application for Waveform {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Waveform, Command<Self::Message>) {
        (
            Waveform {
                state: State::new()
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
            Message::Tick(instant) => self.state.update(instant),
        };
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        canvas::Canvas::new(&self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}

fn main() -> iced::Result {
    println!("Exploring the 🌊's...");

    Waveform::run(Settings::default())    
}
