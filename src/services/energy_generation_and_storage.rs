use dioxus::prelude::*;

#[component]
pub fn EnergyGenerationStorage() -> Element {
    rsx! {
        style {{include_str!("../css_files/energy_generation_and_storage.css")}}
        div { class: "container",
            h1 { class: "main-header", "Welcome to Tesero Sol: Your Clean Energy and Energy Storage Experts" }
            p { class: "sub-header", "Powering a Sustainable Future" }
            p { class: "intro-text", "Tesero Sol is dedicated to accelerating the transition to a low-carbon future. Our 'Everything as a Grid' approach transforms how power is distributed, stored, and consumed, making energy systems more flexible and efficient." }

            h2 { class: "section-title", "Clean Energy Solutions" }
            h3 { class: "subsection-title", "Renewable Energy Transition" }
            p { class: "paragraph", "Renewable energy is on the rise, with electricity demand expected to soar by 2050. We help manage this shift by creating flexible, bi-directional power networks." }

            h3 { class: "subsection-title", "Cleaner Power Initiatives" }
            p { class: "paragraph", "Businesses and consumers are increasingly sourcing renewable electricity. We facilitate trading of self-generated clean electricity to lower energy costs and support grid balancing." }

            h3 { class: "subsection-title", "Energy Independence" }
            p { class: "paragraph", "Homes and businesses are becoming self-sufficient power producers. With solar arrays, wind turbines, microgrids, and battery storage, our clients generate, store, and consume their own energy, selling excess back to the grid." }

            h3 { class: "subsection-title", "Smart Energy Management" }
            p { class: "paragraph", "Digital innovation allows for smarter energy management. By transforming data into actionable insights, we help maximize efficiency and manage energy footprints." }

            h2 { class: "section-title", "Our Installation Services" }
            ul { class: "list",
                li { class: "list-item", "Site Assessment: We determine the best clean energy solutions for your property." }
                li { class: "list-item", "Customized Design: We tailor systems to maximize efficiency and savings." }
                li { class: "list-item", "Professional Installation: Certified technicians ensure safe, correct setup." }
                li { class: "list-item", "System Integration: Seamless integration with existing electrical infrastructure." }
                li { class: "list-item", "Maintenance and Support: Ongoing support to keep your system running smoothly." }
            }

            h2 { class: "section-title", "Different Sized EV Chargers" }
            p { class: "paragraph",
                "We offer a variety of EV chargers to suit different needs and budgets. Our chargers come in various sizes and capacities, ensuring that you have the perfect fit for your property."
            }
            ul { class: "list",
                li { class: "list-item", "Level 1 Chargers: These are basic chargers that use a standard household outlet, ideal for overnight charging at home." }
                li { class: "list-item", "Level 2 Chargers: Offering faster charging times, these chargers require a dedicated 240V outlet and are perfect for both residential and commercial installations." }
                li { class: "list-item", "DC Fast Chargers: These high-capacity chargers provide rapid charging and are best suited for commercial properties, public charging stations, and fleet operations." }
            }
            p { class: "paragraph",
                "No matter the size or type of charger you choose, Tesero Sol ensures high-quality installation and ongoing support to keep your EV charging infrastructure running smoothly."
            }

            h2 { class: "section-title", "Contact Us" }
            p { class: "contact-text", "Join the energy transition with Tesero Sol. Contact us today to learn more about our clean energy and energy storage solutions." }

            p { class: "closing-text", "Thank you for visiting Tesero Sol. Together, we can power a sustainable future." }
        }
    }
}
