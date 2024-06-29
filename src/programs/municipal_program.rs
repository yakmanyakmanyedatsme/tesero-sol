use dioxus::prelude::*;

#[component]
pub fn MunicipalMicrogridProgram() -> Element {
    rsx! {
        style { {include_str!("../css_files/munipal_program_style.css")} }
        div {class: "apply-container",
        h1 { class: "header-med", "Apply for Municipal Microgrid Program" }
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
                src: "https://www.canva.com/design/DAGJLN5m8AQ/YnskYbqCyl3Nw0WokLpl1Q/view?embed",
            }
    }
}
