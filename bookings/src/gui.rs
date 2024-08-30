use crate::types::{Booking, Bookings, Msg};
use iced::widget::{button, checkbox, column, row, text, text_input};
use iced::{Element, Sandbox, Theme};
use std::process::exit;

impl Sandbox for Bookings {
    type Message = Msg;

    fn new() -> Self {
        Self {
            booking: Booking {
                id: 0,
                area: 0,
                weeks: 0,
                rooms: 0,
                address: String::new(),
                date: String::new(),
                phone: String::new(),
                owner: String::new(),
                alarm: false,
                maintenance: false,
            },
            bookings: Vec::new(),
            tbox: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Booking System v1")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::Submit => {
                self.bookings.push(self.booking.clone());
                self.update(Msg::Clear);
                self.tbox = String::from("Booking saved");
            }
            Msg::Update => {
                let mut flag = false;
                let id = self.booking.id;
                for i in 0..self.bookings.len() {
                    if self.bookings[i].id == id {
                        flag = true;
                        self.bookings[i] = self.booking.clone();
                        self.tbox = String::from("Booking updated")
                    }
                }
                if !flag {
                    self.tbox = String::from("Unknown Booking ID")
                }
            }
            Msg::List => {
                self.tbox = String::new();
                for b in &self.bookings {
                    self.tbox += &format!("{}\n", b.get_booking());
                }
            }
            Msg::Clear => {
                self.booking.clear();
                self.tbox = String::new();
            }
            Msg::Exit => exit(0),
            Msg::ID(s) => self.booking.id = s.trim().parse().unwrap_or_default(),
            Msg::Area(s) => self.booking.area = s.trim().parse().unwrap_or_default(),
            Msg::Weeks(s) => self.booking.weeks = s.trim().parse().unwrap_or_default(),
            Msg::Rooms(s) => self.booking.rooms = s.trim().parse().unwrap_or_default(),
            Msg::Address(s) => self.booking.address = s,
            Msg::Date(s) => self.booking.date = s,
            Msg::Phone(s) => self.booking.phone = s,
            Msg::Name(s) => self.booking.owner = s,
            Msg::Alarm(b) => self.booking.alarm = b,
            Msg::Maintenance(b) => self.booking.maintenance = b,
        }
    }

    fn view(&self) -> Element<Msg> {
        column![
            column![
                text_input("ID (number)", &self.booking.get_id()).on_input(Msg::ID),
                text_input("Garden Area (number)", &self.booking.get_area()).on_input(Msg::Area),
                text_input("Weeks (number)", &self.booking.get_weeks()).on_input(Msg::Weeks),
                text_input("Rooms (number)", &self.booking.get_rooms()).on_input(Msg::Rooms),
                text_input("Address", &self.booking.address).on_input(Msg::Address),
                text_input("Date", &self.booking.date).on_input(Msg::Date),
                text_input("Number", &self.booking.phone).on_input(Msg::Phone),
                text_input("Owner", &self.booking.owner).on_input(Msg::Name),
            ]
            .spacing(5),
            row![
                checkbox("Alarm Check", self.booking.alarm).on_toggle(Msg::Alarm),
                checkbox("Pool Maintenance", self.booking.maintenance).on_toggle(Msg::Maintenance),
            ]
            .spacing(5),
            row![
                button("Submit").on_press(Msg::Submit),
                button("Update").on_press(Msg::Update),
                button("List").on_press(Msg::List),
                button("Clear").on_press(Msg::Clear),
                button("Exit").on_press(Msg::Exit),
            ]
            .spacing(5),
            text(&self.tbox),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
