use fundsp::hacker::*;
use iced::widget::{button, column, horizontal_rule, text};
use iced::executor;
use iced::{Alignment, Application, Command, Element, Settings, Theme};


#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}


struct Counter {
    value: i32,
}


impl Application for Counter {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Counter, Command<Self::Message>) {
        (
            Counter{value: 0},
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
            Message::IncrementPressed => self.value += 10,
            Message::DecrementPressed => self.value -= 10
        };
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            text("Incremening Button"),
            horizontal_rule(10),
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed),
        ]
        .padding(50)
        .align_items(Alignment::Center)
        .into()
    }
}


fn main() -> iced::Result {
    println!("Exploring the 🌊's...");

    Counter::run(Settings::default())    
}
