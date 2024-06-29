use dioxus::prelude::*;

pub mod business;
pub mod municipal_program;
pub mod residential;
use business::BrokerServiceCanva;
use municipal_program::MunicipalMicrogridProgram;
use residential::ResidentialProgram;
#[component]
pub fn Programs() -> Element {
    let mut active_tab: Signal<u32> = use_signal(|| 0);
    rsx! {
        style { {include_str!("../css_files/program_style.css")} }
        ul { class: "tabs",
            li {
                class: if active_tab() == 0 { "active" } else { "" },
                onclick: move |_| active_tab.set(0),
                "Residential/SME Batteries",
            },
            li {
                class: if active_tab() == 1 { "active" } else { "" },
                onclick: move |_| active_tab.set(1),
                "Municipal energy System"
            },
            li {
                class: if active_tab() == 2 { "active" } else { "" },
                onclick: move |_| active_tab.set(2),
                "Broker Service"
            },
        }

        div { class: "contents",
            if active_tab() == 0 {
                Batteries {}
                ResidentialProgram {}
            }
            if active_tab() == 1 {
                MunicipalMicrogrid {}
                MunicipalMicrogridProgram {}
            }
            if active_tab() == 2 {
                BrokerService {}
                BrokerServiceCanva {}
            }
        }
    }
}

#[component]
pub fn Batteries() -> Element {
    rsx! {
        div {
            class: "box show",
            div {
            h3 {"Battery Program:"}
            p {class: "mission_statement","Empowering Southern California with Clean, Resilient, and Sustainable Energy"}
            p {class: "justify_text",
            "Residential battery program for home owners in southern california. Introducing our energy relief program. Where residences can receive significant
             energy savings with the savings on the battery purchase being up to 8,050 USD for a 12,000 dollar battery"}
            }
        }
    }
}

#[component]
pub fn MunicipalMicrogrid() -> Element {
    rsx! {
        div {
            class: "box show",
            div {
                h3 {"Microgrid Program"}
                p {class: "mission_statement","Empowering Southern California with Clean, Resilient, and Sustainable Energy"}
                p {class: "justify_text",
                "Residential battery program for home owners in southern california. Introducing our energy relief program. Where residences can receive significant
                 energy savings with the savings on the battery purchase being up to 8,050 USD for a 12,000 dollar battery"}
            }
        }
    }
}

#[component]
pub fn BrokerService() -> Element {
    rsx! {
        div {
            class: "box show",
            div {
                h3 {"Brokerage Collaboration"}
                p {class: "mission_statement","Empowering Southern California with Clean, Resilient, and Sustainable Energy"}
                p {class: "justify_text",
                "Residential battery program for home owners in southern california. Introducing our energy relief program. Where residences can receive significant
                 energy savings with the savings on the battery purchase being up to 8,050 USD for a 12,000 dollar battery"}
                }
            }
    }
}
