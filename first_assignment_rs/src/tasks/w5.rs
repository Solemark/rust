use dioxus::prelude::*;
pub fn w5() {
    dioxus_desktop::launch(app)
}

fn app(cx: Scope) -> Element {
    let name: &UseState<String> = use_state(cx, || "".to_string());
    let mark: &UseState<String> = use_state(cx, || "".to_string());
    let total: &UseState<i32> = use_state(cx, || 0);
    let count: &UseState<i32> = use_state(cx, || 0);
    let avg: &UseState<i32> = use_state(cx, || 0);
    cx.render(rsx! {
            table {
                tr {
                    td {
                        label {"Name:"}
                    }
                    td {
                        input {
                            name: "name",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value.clone())
                        }
                    }
                }
                tr {
                    td {
                        label {"Mark:"}
                    }
                    td {
                        input {
                            name: "mark",
                            value: "{mark}",
                            oninput: move |evt| mark.set(evt.value.clone())
                        }
                    }
                }
                tr {
                    td {
                        label {"Total:"}
                    }
                    td {
                        label {"{total}"}
                    }
                }
                tr {
                    td {
                        label {"Count:"}
                    }
                    td {
                        label {"{count}"}
                    }
                }
                tr {
                    td {
                        label {"Average:"}
                    }
                    td {
                        label {"{avg}"}
                    }
                }
                tr {
                    button {
                        onclick: move |_| {
                            let mark_: i32 = mark.get().trim().parse().expect("unable to convert to i32");
                            let total_: i32 = total.get() + mark_;
                            let count_: i32 = count.get() + 1;
                            total.set(total_);
                            count.set(count_);
                            avg.set(total_ / count_);
                            name.set("".to_string());
                            mark.set("".to_string());
                        },
                        "submit",
                    }
                }
            }
        })
}
