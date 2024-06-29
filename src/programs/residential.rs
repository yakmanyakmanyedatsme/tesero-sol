use dioxus::prelude::*;
pub fn ResidentialProgram() -> Element {
    rsx! {
        style {{include_str!("../css_files/residential_program_style.css")} }
        div { class: "container-battery-program",
            h1 { class: "header-med", "Apply for Residential Battery Program" }
            button { class: "start-button", "Start" }
            h1 {class: "font-size: 1.5rem;", "Needed Documentation"}
            ul { class: "checklist",
                li { "Proof of identity" }
                li { "Proof of residence" }
                li { "Last 2 months of energy bills" }
                li { "Proof of income" }
            }
            image_sidebyside {}
        }
        div { class: "info-figure",
            figure {
            img { style: "width: 75vh; height: 80vh;", src: "https://i.ibb.co/v3dPv7k/residential-battery-info.png", alt: "Image 1 description" }
            }
            figure {
            img {  style: "width: 55vh; height: 80vh;", src:"https://i.ibb.co/GM90JNQ/program.png", alt: "Image 1 description" }
                figcaption {
                    "The pilot program is competitively priced with a net cost of $3,950 per unit after rebates and tax credits,
                            with a total investment of $1.2 million. The future program expands the initiative, 
                            promising even greater returns on investment."
                }
            }
        }
        energy_storage_solicitation {}
    }
}

#[component]
pub fn image_sidebyside() -> Element {
    rsx! {
        div { class: "images-container",
                img { style: "width: 60vh; height: 40vh", src: "https://i.ibb.co/FsLkhjL/interior-battery.webp", alt: "Image 1 description" }
                img { style: "width: 60vh; height: 40vh", src: "https://i.ibb.co/mTJ6KKz/exterior-battery.webp", alt: "Image 2 description" }
        }
    }
}

#[component]
pub fn energy_storage_solicitation() -> Element {
    rsx! {
        div { class: "solicitation-container",
            h1 { "Solicitation for Innovative Home Energy Storage Solutions" }
            p { class: "intro",
                "QPO Energy LLC, a trailblazer in the energy storage sector,
                 invites Public Utilities Districts (PUDs) to collaborate in
                a pioneering pilot program aimed at revolutionizing home
                energy management. Founded in 2020 in Tualatin, Oregon, 
                by Joseph Lu, a luminary with a rich legacy in the energy 
                storage industry, QPO Energy is at the forefront of deploying
                 safe, efficient, and reliable energy storage solutions."
            }
            h2 { "About the Program:" }
            p {
                "QPO Energy proposes an ambitious plan to deploy 100 residential
                 Battery Energy Storage Systems (BESS) across five cities, with
                 a further expansion to 700 BESS in subsequent phases. These 
                 state-of-the-art systems are designed for seamless integration,
                  offering a range of operational modes tailored for energy efficiency, backup power, and peak shaving, thereby promising significant cost savings and enhanced energy independence for homeowners."
            }
            h2 { "Key Features:" }
            ul {
                li { "Utilizes advanced LiFePO4 battery chemistry with over 7000 cycles and a 10-year warranty." }
                li { "The Meter Home Energy System, blending aesthetics with functionality, ensures a sophisticated addition to any living space." }
                li { "Comprehensive operational modes including Storm Mode, Backup Power, and Utility Aggregation, among others, for optimal energy utilization." }
                li { "Flexible interconnect technology allows for significant integration and aggregation of energy storage assets." }
                li { "Pilot program incentives include a substantial rebate, a 30% federal tax credit, and an attractive net cost after rebates." }
            }
            h2 { "Investment and Rebates:" }
        }
    }
}
