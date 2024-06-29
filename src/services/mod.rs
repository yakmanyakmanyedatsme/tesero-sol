pub mod application;
pub mod batteries;
pub mod energy_generation_and_storage;
pub mod ev_chargers;
pub mod solar;

use application::Application;
use batteries::BatteryService;
use dioxus::prelude::*;
use energy_generation_and_storage::EnergyGenerationStorage;
use ev_chargers::EvChargerService;
use solar::SolarService;

#[component]
pub fn Services() -> Element {
    let mut rendered_program_service: Signal<u32> = use_signal(|| 1);
    rsx! {
        style { { include_str!("../css_files/service_style.css") } }
        div{ class: "top-container",
        p {class: "top-statement",
            "Our expert residential solar contractors in New York are ready to help you transition to cleaner, more reliable energy at home."
        }
        h2 {class: "section-title",
         "Ensuring the Best Residential Solar System in New York – Our Process"
        }
        }
        ImageGrid {}
        div {
            div { class: "button_container",
                button { class: "button", onclick: move |_| rendered_program_service.set(1), "Batteries" }
                button { class: "button", onclick: move |_| rendered_program_service.set(2), "EV Chargers" }
                button { class: "button", onclick: move |_| rendered_program_service.set(3), "Solar Panels" }
                button { class: "button", onclick: move |_| rendered_program_service.set(4), "Energy Generation & Storage" }
            }
            if rendered_program_service() == 1 {
               BatteryService {}
            }
            if rendered_program_service() == 2 {
                EvChargerService {}

            }
            if rendered_program_service() == 3 {
                SolarService {}
            }
            if rendered_program_service() == 4 {
                EnergyGenerationStorage {}
            }
        }
    }
}

#[component]
pub fn ImageGrid() -> Element {
    rsx! {
        ul {
            class: "image-grid",
            li {
                class: "image-grid__item1",
                a {
                    href: "#",
                    class: "grid-item",
                    div { class: "grid-item__name", "Initial Consultation:" }
                    div {
                        class: "grid-item__image",
                        " We begin with a consultation to determine if a residential
                         solar system suits your needs."
                    }
                    div { class: "grid-item__hover" }
                }
            }
            li {
                class: "image-grid__item2",
                a {
                    href: "#",
                    class: "grid-item",
                    div { class: "grid-item__name", "On-Site Survey:" }
                    div {
                        class: "grid-item__image",
                        "A detailed survey of your home gauges its solar
                         potential, followed by a comprehensive cost-benefit analysis."
                    }
                    div { class: "grid-item__hover" }
                }
            }
            li {
                class: "image-grid__item3",
                a {
                    href: "#",
                    class: "grid-item",
                    div { class: "grid-item__name", "Permitting:" }
                    div {
                        class: "grid-item__image",
                        "We handle all the permitting, reducing your stress
                         and ensuring a smooth process."
                    }
                    div { class: "grid-item__hover" }
                }
            }
            li {
                class: "image-grid__item4",
                a {
                    href: "#",
                    class: "grid-item",
                    div { class: "grid-item__name", "Installation:" }
                    div {
                        class: "grid-item__image",
                        "Upon approval, our qualified residential solar contractors
                         will install your system, typically completing the installation within
                          a day."
                    }
                    div { class: "grid-item__hover" }
                }
            }
        }
    }
}
