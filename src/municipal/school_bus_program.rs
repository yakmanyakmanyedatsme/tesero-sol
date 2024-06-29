use dioxus::prelude::*;

#[component]
pub fn CleanEnergyFinProgram() -> Element {
    rsx! {
        style { {include_str!("../css_files/clean_energy_styles.css") }}
        div {
            class: "clean_energy_section",
            h2 {
                class: "section_title",
                "Title 17 Clean Energy Financing Overview"
            }
            p {
                class: "paragraph-main",
                "The Title 17 Clean Energy Financing Program provides financing for U.S. projects
                 that support clean energy deployment and infrastructure reinvestment to reduce 
                 greenhouse gas emissions. Administered by the Loan Programs Office (LPO), it 
                 offers loans and guarantees for innovative energy technologies and infrastructure 
                 upgrades, focusing on large-scale sustainable energy solutions."
            }
            p {
                class: "paragraph",
                "Benefits of Partnering with LPO:"
            }
            ul {
                class: "list",
                li { "Access to Capital: For large-scale projects." }
                li { "Technical Due Diligence: Thorough project analysis." }
                li { "Flexible Financing: Various financing options." }
                li { "Specialized Expertise: In-house financial, technical, legal, and environmental professionals." }
            }
            h2 {
                class: "section_title",
                "Project Categories"
            }
            ul {
                class: "list",
                li { "Innovative Energy: Projects with new or significantly improved technology not yet widely commercialized in the U.S." }
                li { "Innovative Supply Chain: Projects employing new technology in manufacturing clean energy technology." }
                li { "State Energy Financing Institution (SEFI)-Supported: Projects supported by state agencies for clean energy deployment." }
                li { "Energy Infrastructure Reinvestment (EIR): Projects repurposing or upgrading energy infrastructure to reduce pollutants." }
            }
            h2 {
                class: "section_title",
                "Financing Options"
            }
            ul {
                class: "list",
                li { "Direct Loans: From U.S. Treasury’s Federal Financing Bank (FFB), backed by DOE guarantees." }
                li { "Partial Guarantees: DOE partial guarantees of commercial debt." }
                li { "Interest Rate: Based on the U.S. Treasury curve plus a spread and a risk-based charge, with potential reductions for certain projects." }
            }
            h2 {
                class: "section_title",
                "Eligibility Criteria"
            }
            p {
                class: "paragraph",
                "Projects must meet seven key criteria and additional specific requirements per category."
            }
            h2 {
                class: "section_title",
                "Costs and Fees"
            }
            ul {
                class: "list",
                li { "Application Fees: None." }
                li { "External Advisor Fees: Ranges from $1 million to $3 million." }
                li { "Facility Fee: 0.6% of the principal up to $2 billion, plus 0.1% beyond $2 billion." }
                li { "Maintenance Fees: $150,000 to $200,000 per year, up to $500,000 depending on complexity." }
            }
            p {
                class: "paragraph",
                "LPO typically finances projects of $100 million or more."
            }
            h2 {
                class: "section_title",
                "Application Process"
            }
            ul {
                class: "list",
                li { "Pre-Application" }
                li { "Application & Review" }
                li { "Due Diligence" }
                li { "Conditional Commitment" }
                li { "Financial Close" }
                li { "Monitoring" }
            }
            p {
                class: "paragraph",
                "The process duration ranges from six months to over a year."
            }
            h2 {
                class: "section_title",
                "Next Steps"
            }
            p {
                class: "paragraph",
                "Request a no-cost pre-application consultation. Explore other funding opportunities through the DOE's Office of Clean Energy Demonstrations (OCED)."
            }
        }
    }
}
