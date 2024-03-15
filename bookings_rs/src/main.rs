use dioxus::prelude::*;
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Booking {
    booking_id: String,
    garden_area: String,
    num_weeks: String,
    num_rooms: String,
    address: String,
    booking_date: String,
    contact_number: String,
    property_owner_name: String,
    security_alarm_check: bool,
    pool_maintenance: bool,
}

fn app(cx: Scope) -> Element {
    let booking_id: &UseState<String> = use_state(cx, || "".to_string());
    let garden_area: &UseState<String> = use_state(cx, || "".to_string());
    let num_weeks: &UseState<String> = use_state(cx, || "".to_string());
    let num_rooms: &UseState<String> = use_state(cx, || "".to_string());
    let address: &UseState<String> = use_state(cx, || "".to_string());
    let booking_date: &UseState<String> = use_state(cx, || "".to_string());
    let contact_number: &UseState<String> = use_state(cx, || "".to_string());
    let property_owner_name: &UseState<String> = use_state(cx, || "".to_string());
    let security_alarm_check: &UseState<bool> = use_state(cx, || false);
    let pool_maintenance: &UseState<bool> = use_state(cx, || false);
    let booking_list: &UseState<Vec<Booking>> = use_state(cx, || vec![]);
    cx.render(rsx! {
        table {
            tr {
                td {
                    label {"Booking ID:"}
                }
                td {
                    input {
                        value: "{booking_id}",
                        oninput: move |e| booking_id.set(e.value.clone()),
                    }
                }
            }
            tr {
                td {
                    label {"Garden Area:"}
                }
                td {
                    input {
                        value: "{garden_area}",
                        oninput: move |e| garden_area.set(e.value.clone()),
                    }
                }
            }
            tr {
                td {
                    label {"Number of Weeks:"}
                }
                td {
                    input {
                        value: "{num_weeks}",
                        oninput: move |e| num_weeks.set(e.value.clone()),
                    }
                }
            }
            tr {
                td {
                    label {"Number of Rooms:"}
                }
                td {
                    input {
                        value: "{num_rooms}",
                        oninput: move |e| num_rooms.set(e.value.clone()),
                    }
                }
            }
            tr {
                td {
                    label {"Address:"}
                }
                td {
                    input {
                        value: "{address}",
                        oninput: move |e| address.set(e.value.clone()),
                    }
                }
            }
            tr {
                td {
                    label {"Booking Date:"}
                }
                td {
                    input {
                        value: "{booking_date}",
                        oninput: move |e| booking_date.set(e.value.clone()),
                    }
                }
            }
            tr {
                td {
                    label {"Contact Number:"}
                }
                td {
                    input {
                        value: "{contact_number}",
                        oninput: move |e| contact_number.set(e.value.clone()),
                    }
                }
            }
            tr {
                td {
                    label {"Property Owner Name:"}
                }
                td {
                    input {
                        value: "{property_owner_name}",
                        oninput: move |e| property_owner_name.set(e.value.clone()),
                    }
                }
            }
            tr {
                td {
                    label {"Security Alarm Check:"}
                }
                td {
                    input {
                        r#type: "checkbox",
                        value: "{security_alarm_check}",
                        oninput: move |_| security_alarm_check.set(!security_alarm_check),
                    }
                }
            }
            tr {
                td {
                    label {"Pool Maintenance:"}
                }
                td {
                    input {
                        r#type: "checkbox",
                        value: "{pool_maintenance}",
                        oninput: move |_| pool_maintenance.set(!pool_maintenance),
                    }
                }
            }
            tr {
                button {
                    onclick: move |_| {
                        let mut bookings: Vec<Booking> = booking_list.get().to_vec();
                        bookings.push(Booking {
                                booking_id: booking_id.get().to_string(),
                                garden_area: garden_area.get().to_string(),
                                num_weeks: num_weeks.get().to_string(),
                                num_rooms: num_rooms.get().to_string(),
                                address: address.get().to_string(),
                                booking_date: booking_date.get().to_string(),
                                contact_number: contact_number.get().to_string(),
                                property_owner_name: property_owner_name.get().to_string(),
                                security_alarm_check: *security_alarm_check.get(),
                                pool_maintenance: *pool_maintenance.get()
                            }
                        );
                        booking_list.set(bookings);
                    },
                    "Submit",
                }
                button {
                    onclick: move |_| {
                        booking_id.set("".to_string());
                        garden_area.set("".to_string());
                        num_weeks.set("".to_string());
                        num_rooms.set("".to_string());
                        address.set("".to_string());
                        booking_date.set("".to_string());
                        contact_number.set("".to_string());
                        property_owner_name.set("".to_string());
                        security_alarm_check.set(false);
                        pool_maintenance.set(false);
                    },
                    "Clear",
                },
            }
        }
    })
}

fn main() {
    dioxus_desktop::launch(app);
}
