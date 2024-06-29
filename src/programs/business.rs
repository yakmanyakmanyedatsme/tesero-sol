use dioxus::prelude::*;

#[component]
pub fn BrokerServiceCanva() -> Element {
    rsx! {
        style { {include_str!("../css_files/broker_service_style.css")} }
        div {class: "apply-container",
        h1 { class: "header-med", "Apply for Residential Battery Program" }
            button { class: "start-button", "Start" }
            h1 {class: "font-size: 1.5rem;", "Needed Documentation"}
            ul { class: "checklist",
                li { "Proof of identity" }
                li { "Proof of residence" }
                li { "Last 2 months of energy bills" }
                li { "Proof of income" }
            }
        }
            iframe {
                style: "width: 50%; height: 125vh; scale:2; margin-left: 25%; padding: 0; margin-top:70vh; ",
                src: "https://www.canva.com/design/DAF1dxTzCFE/PCg73R8W7TmW5MK4dKHDOA/view?embed",
            }
    }
}
