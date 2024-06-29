use dioxus::prelude::*;

#[component]
pub fn BatteryService() -> Element {
    rsx! {
        style {{include_str!("../css_files/batteries_style.css")}}
    div { class: "container",
        h1 { class: "header-title", "Residential Battery Installation in Southern California" }
        p { class: "mission-statement", "Empowering Southern California with Clean, Resilient, and Sustainable Energy" }

        h2 { class: "section-title", "Why Choose a Battery System?" }
        ul { class: "list",
            li { class: "list-item", "Energy Independence: Store energy generated by your solar panels during the day and use it at night." }
            li { class: "list-item", "Cost Savings: Reduce your reliance on the utility grid and save on energy bills." }
            li { class: "list-item", "Backup Power: Ensure your home remains powered during outages, maintaining safety and comfort." }
        }

        h2 { class: "section-title", "Our Battery Program" }
        p { "Our residential battery program is designed to offer significant savings and reliability:"
            ul { class: "list",
                li { class: "list-item", "Substantial Savings: Receive up to $8,050 in savings on a $12,000 battery purchase." }
                li { class: "list-item", "Reliable Energy: Ensure your home has continuous power during outages." }
            }
        }

        h2 { class: "section-title", "Apply for the Residential Battery Program" }
        p { class: "instructions", "Applying for our program is simple. You'll need:" }
        ul { class: "list",
            li { class: "list-item", "Proof of identity" }
            li { class: "list-item", "Proof of residence" }
            li { class: "list-item", "Last 2 months of energy bills" }
            li { class: "list-item", "Proof of income" }
        }

        h2 { class: "section-title", "Innovative Battery Solutions" }
        p { "Tesero Sol is a certified installer of the latest battery technologies, ensuring you receive the best in energy storage solutions:"
            ul { class: "list",
                li { class: "list-item", "Tesla Powerwall: A rechargeable home battery system that stores solar energy for use at night or during power outages." }
                li { class: "list-item", "SunPower® SunVault™: A storage system providing up to three days of power, ensuring peace of mind even in bad weather." }
            }
        }

        h2 { class: "section-title", "Efficient Power Management" }
        p { "Our systems are designed to maximize efficiency:"
            ul { class: "list",
                li { class: "list-item", "Daytime Energy Generation: Your solar panels generate electricity, which is either used immediately or stored in the battery." }
                li { class: "list-item", "Nighttime Energy Use: The system draws power from the battery, only using the grid when the battery is depleted." }
            }
        }

        h2 { class: "section-title", "Contact Us" }
        p { "Ready to take control of your energy? Contact Tesero Sol today:" }
        ul { class: "list",
            li { class: "list-item", "Phone: 844-986-3703" }
            li { class: "list-item", "Email: info@teserosol.com" }
        }
        p { "Empower your home with Tesero Sol's advanced battery solutions and enjoy the benefits of energy independence, cost savings, and reliable power." }

        h2 { class: "section-title", "Testimonials" }
        ul { class: "list",
            li { class: "list-item", "Trudi Schwert: \"It was a natural progression for us. We've always used our natural resources wisely. With Tesero Sol, we are now energy independent.\"" }
            li { class: "list-item", "Donald C.: \"The lights flickered for a half cycle, and I said to my wife, the grid power just went out. Thanks to Tesero Sol, we didn't even notice.\"" }
        }

        h2 { class: "section-title", "About Tesero Sol" }
        ul { class: "list",
            li { class: "list-item", "Experience: Since 2003, we've helped homeowners and businesses utilize solar power to reduce their carbon footprint and save money." }
            li { class: "list-item", "Family-Owned: Personalized customer service from a local, family-owned team." }
            li { class: "list-item", "Competitive Pricing: Investing in clean energy shouldn't cost an arm and a leg. Our competitive pricing keeps your project within budget." }
            li { class: "list-item", "NABCEP-Certified: Our knowledgeable solar experts ensure your system is safely and properly installed." }
        }
        p { "Join the movement towards a cleaner, more sustainable future with Tesero Sol's residential battery solutions." }
    }}
}
