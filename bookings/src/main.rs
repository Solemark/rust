mod booking;
mod gui;
use iced::{Sandbox, Settings};

pub fn main() -> iced::Result {
    gui::Bookings::run(Settings::default())
}
