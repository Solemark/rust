#[derive(Clone)]
pub(crate) struct Booking {
    pub(crate) booking_id: usize,
    pub(crate) garden_area: f32,
    pub(crate) num_weeks: u32,
    pub(crate) num_rooms: u32,
    pub(crate) address: String,
    pub(crate) booking_date: String,
    pub(crate) contact_number: String,
    pub(crate) property_owner_name: String,
    pub(crate) security_alarm_check: bool,
    pub(crate) pool_maintenance: bool,
}
