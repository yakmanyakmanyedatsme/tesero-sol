use dioxus::prelude::*;

#[component]
pub fn NationalEvInfra() -> Element {
    rsx! {
    style {{include_str!("../css_files/nevi_style.css")}}
    div {
        class: "nevi_section",
            h2 {
                class: "section_title",
                "NEVI Formula Program Funding Features"
            }
            h3 {
                class: "subsection_title",
                "A. Authorization Levels"
            }
            p {
            class: "paragraph-main",
                "The Bipartisan Infrastructure Law (BIL) appropriates a total of $5.0 billion for the NEVI Formula Program over the period of fiscal years 2022 through 2026. Table 1 shows the NEVI Formula Program levels by fiscal year."
            }
            table {
                class: "funding_table",
                thead {
                    tr {
                        th { "Fiscal Year" }
                        th { "2022" }
                        th { "2023" }
                        th { "2024" }
                        th { "2025" }
                        th { "2026" }
                    }
                }
                tbody {
                    tr {
                        td { "Advance Appropriation (General Fund)" }
                        td { "$1.000 B" }
                        td { "$1.000 B" }
                        td { "$1.000 B" }
                        td { "$1.000 B" }
                        td { "$1.000 B" }
                    }
                }
            }
            h3 {
                class: "subsection_title",
                "B. NEVI Formula Program"
            }
            ul {
                class: "list",
                li { "Type of Budget Authority: Current and advance appropriations from the General Fund." }
                li { "Period of Availability: Available until expended." }
                li {
                    "Pre-Apportionment Set-Asides:"
                    ul {
                        class: "nested-list",
                        li { "For FY22 only, the BIL sets aside up to $300 million for the Departments of Transportation and Energy to establish a Joint Office for guidance and technical assistance." }
                        li { "For each year of FY22-26, 10% of EV Formula funding is set aside for grants to states and localities requiring additional assistance." }
                        li { "Up to 1.5% of annual NEVI Formula Program funds can be used for FHWA operations and administration." }
                    }
                }
                li { "Distribution of Funds: Funds are distributed among states, including DC and Puerto Rico, based on federal-aid highway apportionments and Puerto Rico Highway Program funding. This funding is not subject to any limitation on obligation." }
            }
            h3 {
                class: "subsection_title",
                "C. Federal Share and State/Local Match Requirements"
            }
            ul {
                class: "list",
                li { "Cost-Share: The federal cost-share for NEVI Formula Program projects is 80%, with the remaining 20% provided by private or state funds." }
                li { "Combining Funds: NEVI funds can be combined with other USDOT funding, such as the Congestion Mitigation and Air Quality Improvement (CMAQ) Program, provided total federal cost-share does not exceed 80%." }
            }
            h3 {
                class: "subsection_title",
                "D. Specific Funding Requirements"
            }
            ul {
                class: "list",
                li {
                    "Alternative Fuel Corridors:"
                    ul {
                        class: "nested_list",
                        li { "Funds must be used for EV charging infrastructure along designated Alternative Fuel Corridors (AFCs)." }
                        li { "States should prioritize the Interstate Highway System." }
                        li { "States can adjust nominations for AFCs and use funds on designated corridors on the National Highway System if necessary." }
                        li { "If AFCs are fully built out, states may use funds for public roads or other accessible locations." }
                    }
                }
                li {
                    "Contracting with Private Entities:"
                    ul {
                        class: "nested_list",
                        li { "States can contract private entities for acquisition, installation, operation, and maintenance of EV charging infrastructure." }
                        li { "Private entities may pay the non-Federal share of project costs." }
                        li { "Ownership of EV infrastructure does not need to revert to the state if a private entity is contracted." }
                    }
                }
                li { "Transferability: NEVI funds cannot be transferred to other highway formula programs." }
            }
        }
    }
}
