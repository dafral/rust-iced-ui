use iced::widget::{button, text, column, Column};

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[test]
fn it_counts_properly(){
    let mut counter = Counter { value: 0 };
    counter.update(Message::Increment);
    assert_eq!(counter.value, 1);
    counter.update(Message::Decrement);
    assert_eq!(counter.value, 0);
}


