use crate::booking;
use booking::Booking;
use iced::widget::{button, checkbox, column, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Theme};

pub(crate) struct Bookings {
    booking: Booking,
    bookings: Vec<Booking>,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Submit,
    Clear,
    BookingID(String),
    GardenArea(String),
    NumWeeks(String),
    NumRooms(String),
    Address(String),
    BookingDate(String),
    ContactNumber(String),
    PropertyOwnerName(String),
    SecurityAlarmCheck(bool),
    PoolMaintenance(bool),
}

impl Sandbox for Bookings {
    type Message = Message;

    fn new() -> Self {
        Self {
            booking: Booking {
                booking_id: 0,
                garden_area: 0.0,
                num_weeks: 0,
                num_rooms: 0,
                address: String::new(),
                booking_date: String::new(),
                contact_number: String::new(),
                property_owner_name: String::new(),
                security_alarm_check: false,
                pool_maintenance: false,
            },
            bookings: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Submit => {
                self.bookings.push(self.booking.clone());
                self.update(Message::Clear);
            }
            Message::Clear => {
                self.booking = Booking {
                    booking_id: 0,
                    garden_area: 0.0,
                    num_weeks: 0,
                    num_rooms: 0,
                    address: String::new(),
                    booking_date: String::new(),
                    contact_number: String::new(),
                    property_owner_name: String::new(),
                    security_alarm_check: false,
                    pool_maintenance: false,
                }
            }
            Message::BookingID(s) => self.booking.booking_id = s.parse::<usize>().unwrap_or(0),
            Message::GardenArea(s) => self.booking.garden_area = s.parse::<f32>().unwrap_or(0.0),
            Message::NumWeeks(s) => self.booking.num_weeks = s.parse::<u32>().unwrap_or(0),
            Message::NumRooms(s) => self.booking.num_rooms = s.parse::<u32>().unwrap_or(0),
            Message::Address(s) => self.booking.address = s,
            Message::BookingDate(s) => self.booking.booking_date = s,
            Message::ContactNumber(s) => self.booking.contact_number = s,
            Message::PropertyOwnerName(s) => self.booking.property_owner_name = s,
            Message::SecurityAlarmCheck(s) => self.booking.security_alarm_check = s,
            Message::PoolMaintenance(s) => self.booking.pool_maintenance = s,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                text("Booking ID:"),
                text_input("", &self.booking.booking_id.to_string()).on_input(Message::BookingID)
            ],
            row![
                text("Garden Area:"),
                text_input("", &self.booking.garden_area.to_string()).on_input(Message::GardenArea)
            ],
            row![
                text("Number of Weeks:"),
                text_input("", &self.booking.num_weeks.to_string()).on_input(Message::NumWeeks)
            ],
            row![
                text("Number of Rooms:"),
                text_input("", &self.booking.num_rooms.to_string()).on_input(Message::NumRooms)
            ],
            row![
                text("Address:"),
                text_input("", &self.booking.address).on_input(Message::Address)
            ],
            row![
                text("Booking Date:"),
                text_input("", &self.booking.booking_date).on_input(Message::BookingDate)
            ],
            row![
                text("Contact Number:"),
                text_input("", &self.booking.contact_number).on_input(Message::ContactNumber)
            ],
            row![
                text("Property Owner Name:"),
                text_input("", &self.booking.property_owner_name)
                    .on_input(Message::PropertyOwnerName)
            ],
            row![
                checkbox("Security Alarm Check", self.booking.security_alarm_check)
                    .on_toggle(Message::SecurityAlarmCheck),
            ],
            row![
                checkbox("Security Alarm Check", self.booking.pool_maintenance)
                    .on_toggle(Message::PoolMaintenance),
            ],
            row![
                button("Submit").on_press(Message::Submit),
                button("Clear").on_press(Message::Clear)
            ]
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
