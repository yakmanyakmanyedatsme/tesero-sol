use dioxus::prelude::*;

#[component]
pub fn EvChargerService() -> Element {
    rsx! {
        div { class: "container",
            style { { include_str!("../css_files/ev_chargers_style.css") } }
            h1 { class: "main-header", "Electric Vehicle Charging: Powering the Future" }
            section { class: "section",
                h2 { class: "section-title", "Embrace the Electric Revolution" }
                p { class: "paragraph",
                    "In today's swiftly evolving landscape, technology, innovation, and regulation are merging to propel the shift towards low-carbon electric vehicles (EVs). This transition is occurring at an unprecedented rate, presenting an ideal opportunity for forward-thinking property owners and managers to prepare for the future. By upgrading your spaces with EV charging stations, you can benefit from attractive tax credits and incentives, ensuring your property is equipped for the demands of tomorrow."
                }
            }
            section { class: "section",
                h2 { class: "section-title", "Optimize Your Benefits with Tesero Sol" }
                p { class: "paragraph",
                    "At Tesero Sol, we excel in helping you harness the full potential of EV charging integration. Our expertise guarantees you receive the most advantageous short-term and long-term benefits. From detailed site evaluations to flawless installation and ongoing support, we offer a comprehensive solution for all your EV charging needs."
                }
            }
            section { class: "section",
                h2 { class: "section-title", "Why Choose Tesero Sol for Your EV Charging Installation?" }
                ul { class: "list",
                    li { class: "list-item", "Professional Consultation: Our team provides tailored consultations to identify the most efficient and cost-effective EV charging solutions for your property." }
                    li { class: "list-item", "Bespoke Solutions: We design and implement EV charging stations customized to your specific needs, ensuring peak performance and user satisfaction." }
                    li { class: "list-item", "Tax Credits and Savings: We guide you through the intricate landscape of tax credits and incentives, maximizing your savings and return on investment." }
                    li { class: "list-item", "Scalability: Our solutions are designed to grow with you, allowing for easy expansion of your EV charging infrastructure as demand increases." }
                    li { class: "list-item", "Commitment to Sustainability: By partnering with Tesero Sol, you contribute to a cleaner, greener future, supporting the global shift towards sustainable transportation." }
                }
            }
            section { class: "section",
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
            }
            section { class: "section",
                h2 { class: "section-title", "Begin Your EV Charging Journey Today" }
                p { class: "paragraph",
                    "The electric vehicle revolution is here, and it's time to become part of it. Equip your property with state-of-the-art EV charging stations and establish yourself as a leader in sustainability and innovation. Contact Tesero Sol today to discover more about our services and how we can assist you in achieving a seamless EV charging integration."
                }
            }
        }
    }
}
