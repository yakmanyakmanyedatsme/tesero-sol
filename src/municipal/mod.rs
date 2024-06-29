pub mod ccas;
pub mod nevi;
pub mod off_road_rebates;
pub mod school_bus_program;

use crate::municipal::ccas::CommunityChoiceAggregators;
use crate::municipal::nevi::NationalEvInfra;
use crate::municipal::off_road_rebates::OffRoadRebates;
use crate::municipal::school_bus_program::CleanEnergyFinProgram;
use dioxus::prelude::*;

#[component]
pub fn MunicipalProgram() -> Element {
    let mut rendered_program: Signal<u32> = use_signal(|| 1);
    rsx! {
        style {{include_str!("../css_files/municipal_programs.css")}}
        div { class: "top-container",
            h1 { class: "top-statement",
                "Let us help you navigate the Federal and State incentives, and evaluate your energy infrastructure options. With grants
                 available for clean energy projects being at historic levels, and the ability for tax-exempt and goverment entites to reveive Direct Pay "
            }
        }
        div {
                class: "paragraph-main-outside",
                "Direct Pay: The Inflation Reduction Act's Direct Pay (Elective Pay) provision allows
                 tax-exempt and governmental entities to receive payments equal to the full
                value of tax credits for clean energy projects. This enables states, local
                governments, Tribes, territories, and nonprofits to actively participate
                in the clean energy economy, reducing costs and advancing environmental 
                justice. Direct Pay covers 12 tax credits, including those for clean
                electricity eneration, community solar projects, EV charging infrastructure,
                 and clean vehicle purchases."
        }
        div { class: "list_container",
           img {
                src: "https://i.ibb.co/SwQSfLC/1-Eligible-Entities-1.png",
                alt: "1-Eligible-Entities-1",
                border: "0",
                style: "justify-content: center; height: 50vh; width: 100vh;"
            }
        }
        div { class: "button_container",
                button { class: "button", onclick: move |_| rendered_program.set(1), "NEVI" }
                button { class: "button", onclick: move |_| rendered_program.set(2), "CCA" }
                button { class: "button", onclick: move |_| rendered_program.set(3), "Clean Heavy Duty" }
                button { class: "button", onclick: move |_| rendered_program.set(4), "Clean Energy Loans" }
        }
        div { class: "button_container",
                button { class: "button", onclick: move |_| rendered_program.set(4), "Apply" }
        }
        if rendered_program() == 1 {
            NationalEvInfra {}
        }
        if rendered_program() == 2 {
            CommunityChoiceAggregators {}
        }
        if rendered_program() == 3 {
            OffRoadRebates {}
        }
        if rendered_program() == 4 {
            CleanEnergyFinProgram {}
        }
    }
}

#[component]
pub fn Ppa() -> Element {
    rsx! {
       div {"Power Purchase Agreement"}
    }
}

#[component]
pub fn PublicPrivatePartnership() -> Element {
    rsx! {
       div {}
    }
}

#[component]
pub fn Procuremen() -> Element {
    rsx! {
        div {}
    }
}
