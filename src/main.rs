use fundsp::hacker::*;
use iced::widget::{button, column, text, Column};
use iced::executor;
use iced::{Application, Command, Element, Settings, Theme};


#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}


// struct Counter {
    // value: i32,
// }
struct Counter;


impl Application for Counter {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Counter, Command<Self::Message>) {
        (Counter, Command::none())
    }

    fn title(&self) -> String {
        String::from("🌊")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        // match message {
            // Message::IncrementPressed => self.value += 1,
            // Message::DecrementPressed => self.value -= 1
        // }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // column![
        //     button("+").on_press(Message::IncrementPressed),
        //     text(self.value).size(50),
        //     button("-").on_press(Message::DecrementPressed),
        // ]
        "Exploring the waves...".into()
    }
}


fn main() -> iced::Result {
    println!("Exploring the 🌊's...");

    Counter::run(Settings::default())    
}
