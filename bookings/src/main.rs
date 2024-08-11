mod gui;
mod types;
use iced::{Sandbox, Settings};

pub fn main() -> iced::Result {
    crate::types::Bookings::run(Settings::default())
}
