use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings, Theme};

pub fn w5() -> iced::Result {
    W5::run(Settings::default())
}

struct W5 {
    name: String,
    mark: u32,
    total: u32,
    avg: u32,
    count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Submit,
    Clear,
    UpdateName(String),
    UpdateMark(String),
}

impl Sandbox for W5 {
    type Message = Message;

    fn new() -> Self {
        Self {
            name: String::new(),
            mark: 0,
            total: 0,
            avg: 0,
            count: 0,
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        String::from("Week 5")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                self.total += self.mark;
                self.count += 1;
                self.update(Message::Clear);
            }
            Message::Clear => {
                self.name = String::new();
                self.mark = 0;
                self.avg = self.total / self.count;
            }
            Message::UpdateName(s) => self.name = s,
            Message::UpdateMark(s) => self.mark = s.parse::<u32>().unwrap_or(0),
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                text("Student Name:"),
                text_input("", &self.name.to_string()).on_input(Message::UpdateName)
            ],
            row![
                text("Student Mark:"),
                text_input("", &self.mark.to_string()).on_input(Message::UpdateMark)
            ],
            row![
                button("Submit").on_press(Message::Submit),
                button("Clear").on_press(Message::Clear)
            ],
            row![text("Total:"), text_input("", &self.avg.to_string())]
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
