use crate::widget::*;
use iced::{
    widget::{button, container, row, text},
    Length, Sandbox,
};
use widget::node::node;
mod widget;
struct Counter {
    // This will be our state of the counter app
    // a.k.a the current count value
    count: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    // Emitted when the increment ("+") button is pressed
    IncrementCount,
    // Emitted when decrement ("-") button is pressed
    DecrementCount,
}

// Implement Sandbox for our Counter
impl Sandbox for Counter {
    // alias our Message enum with the
    // Sandbox's Message type
    type Message = Message;

    fn new() -> Self {
        // initialize the counter struct
        // with count value as 0.
        Self { count: 0 }
    }

    fn title(&self) -> String {
        //define the title for our app
        String::from("Counter App")
    }

    fn update(&mut self, message: Self::Message) {
        // handle emitted messages
        match message {
            Message::IncrementCount => self.count += 1,
            Message::DecrementCount => self.count -= 1,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // create the View Logic (UI)
        let rw = row![
            button("-").on_press(Message::DecrementCount),
            text(self.count),
            button("+").on_press(Message::IncrementCount),
            node(vec!["Moin".to_string()], Vec::new(), text("hallo").into())
        ];
        container(rw)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    Counter::run(iced::Settings::default())
}
