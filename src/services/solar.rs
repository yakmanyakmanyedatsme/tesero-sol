use dioxus::prelude::*;

#[component]
pub fn SolarService() -> Element {
    rsx! {
        style { { include_str!("../css_files/solar_style.css") } }
        SolarPanelsForHome {}
    }
}

#[component]
pub fn SolarPanelsForHome() -> Element {
    rsx! {
        div { class: "container",
            h1 { class: "main-header", "Why You Should Consider Solar Panels for Your Home" }
            p { class: "intro",
                "New York residents face some of the highest electric bills in the nation. By installing solar panels, you can significantly reduce your monthly bill and achieve a strong return on investment over time."
            }
            h2 { class: "section-title", "Benefits of Solar Panels for Homes" }
            ul { class: "list",
                li { class: "list-item", "Cost Reduction: Solar panels can dramatically reduce, or even eliminate, your monthly power bills. With net metering, you can earn credits for any excess electricity you export to the grid." }
                li { class: "list-item", "Efficiency and Clean Energy: Solar panels provide efficient, clean, and safe energy for all your electrical needs." }
                li { class: "list-item", "Long-Term Savings: Installed with a 25-year warranty, solar panels offer an average return on investment of 15% to 25%." }
                li { class: "list-item", "Protection from Rising Energy Costs: Solar energy shields you from ever-increasing utility rates." }
                li { class: "list-item", "Increased Home Value: Homes with solar panels often see an increase in overall value." }
            }
            p { class: "conclusion", "We offer a comprehensive 25-year warranty for your peace of mind. For a stress-free transition to solar energy for your home, get in touch with us today." }
            h2 { class: "section-title", "Installation Process for Rooftop or Canopies" }
            p { class: "installation-process",
                "Our installation process is designed to be smooth and efficient, ensuring minimal disruption to your daily life:"
            }
            ul { class: "list",
                li { class: "list-item", "Site Assessment: We evaluate your roof or canopy structure to determine the best placement for the solar panels." }
                li { class: "list-item", "Design and Permitting: Our team designs a customized solar panel system and handles all the necessary permitting." }
                li { class: "list-item", "Installation: Certified technicians install the solar panels and integrate them with your existing electrical system." }
                li { class: "list-item", "Inspection and Activation: We conduct a thorough inspection to ensure everything is installed correctly and then activate your new solar energy system." }
                li { class: "list-item", "Ongoing Support: We provide ongoing maintenance and support to ensure your system operates at peak efficiency." }
            }
        }
    }
}
