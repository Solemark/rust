pub(crate) struct Bookings {
    pub(crate) booking: Booking,
    pub(crate) bookings: Vec<Booking>,
    pub(crate) tbox: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Booking {
    pub(crate) id: usize,
    pub(crate) area: u32,
    pub(crate) weeks: u32,
    pub(crate) rooms: u32,
    pub(crate) address: String,
    pub(crate) date: String,
    pub(crate) phone: String,
    pub(crate) owner: String,
    pub(crate) alarm: bool,
    pub(crate) maintenance: bool,
}

impl Booking {
    pub(crate) fn get_id(&self) -> String {
        if self.id == 0 {
            return String::new();
        }
        format!("{}", self.id)
    }

    pub(crate) fn get_area(&self) -> String {
        if self.area == 0 {
            return String::new();
        }
        format!("{}", self.area)
    }

    pub(crate) fn get_weeks(&self) -> String {
        if self.weeks == 0 {
            return String::new();
        }
        format!("{}", self.weeks)
    }

    pub(crate) fn get_rooms(&self) -> String {
        if self.rooms == 0 {
            return String::new();
        }
        format!("{}", self.rooms)
    }

    pub(crate) fn get_booking(&self) -> String {
        format!(
            "id: {}, area: {}, weeks: {}, rooms: {}, address: {}, date: {}, phone: {}, owner: {}, alarm: {}, pool: {}",
            self.id,
            self.area,
            self.weeks,
            self.rooms,
            self.address,
            self.date,
            self.phone,
            self.owner,
            self.alarm,
            self.maintenance,
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Msg {
    Submit,
    Update,
    List,
    Clear,
    Exit,
    ID(String),
    Area(String),
    Weeks(String),
    Rooms(String),
    Address(String),
    Date(String),
    Phone(String),
    Name(String),
    Alarm(bool),
    Maintenance(bool),
}
