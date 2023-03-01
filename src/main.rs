use fundsp::hacker::*;
use iced::widget::{button, column, text, Column};


#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}


struct Counter {
    value: i32,
}


impl Counter {
    pub fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("-").on_press(Message::DecrementPressed),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1
        }
    }
}


fn main() -> iced::Result {
    println!("Exploring the 🌊's...");
    
    
}
