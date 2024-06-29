use dioxus::prelude::*;

#[component]
pub fn CommunityChoiceAggregators() -> Element {
    rsx! { CcaOverview {} }
}

#[component]
pub fn CcaOverview() -> Element {
    rsx! {
        style { {include_str!("../css_files/cca_styles.css") }}
        div {
            class: "cca_section",
            h2 {
                class: "section_title",
                "Community Choice Aggregation (CCA) Overview"
            }
            p {
            class: "paragraph-main",
                "Community Choice Aggregation (CCA) programs allow local governments in California to procure electricity for residents and businesses while the existing investor-owned utility (IOU) continues to deliver it and handle billing and maintenance."
            }
            ul {
                class: "list",
                li { "Aggregation: Local governments aggregate demand to procure electricity." }
                li { "IOU Role: IOUs provide transmission, distribution, metering, and billing services." }
                li { "Enrollment: Residents are automatically enrolled but can opt out." }
                li { "Billing: Customers receive one bill from the IOU that includes both IOU and CCA charges." }
                li { "Charges: Includes non-generation IOU charges, Power Charge Indifference Adjustment (PCIA), and Franchise Fee Surcharge (FFS)." }
                li { "Rate Comparison: Joint Rate Comparisons are available for customers to compare CCA and IOU rates." }
                li { "Customer Support: IOUs and CCAs provide support for their respective charges and services." }
                li { "Programs: IOU programs like energy efficiency rebates, CARE, and the California Solar Initiative remain available to CCA customers." }
            }
            p {
                class: "paragraph",
                "CCA programs offer a local alternative for electricity procurement, allowing communities to choose energy sources, often prioritizing renewable energy, which can lead to environmental benefits and potentially lower costs. This local control provides an opportunity for communities to support their sustainability goals, tailor their energy programs to meet local needs, and encourage innovation in the energy sector while still relying on the established infrastructure and reliability of existing utilities."
            }
            h2 {
                class: "section_title",
                "Guide to Register and File Implementation Plan with CPUC for CCA Service"
            }
            p {
                class: "paragraph",
                "If you represent an entity eligible to participate as a CCA provider, follow these steps to notify the CPUC of your intent to become a CCA:"
            }
            ul {
                class: "list",
                li {
                    "Develop an Implementation Plan: Create a foundational document outlining how your CCA will operate, including services provided, geographical area, and compliance strategies."
                }
                li {
                    "Prepare a Statement of Intent: Formally declare your decision to become a CCA and commit to providing energy services per CPUC regulations."
                }
                li {
                    "Register and File an Implementation Plan with the CPUC: Submit your Implementation Plan for CPUC review and approval to officially recognize your entity as a CCA provider."
                }
                li {
                    "Provide Financial Security to SCE: Ensure financial security to protect customers and stabilize the energy market by providing necessary financial commitments to Southern California Edison (SCE)."
                }
                li {
                    "Secure a Certified Scheduling Coordinator: Interface with the California Independent System Operator (CAISO) for energy scheduling and dispatch through a certified Scheduling Coordinator."
                }
                li {
                    "Ensure Resource Adequacy: Demonstrate the ability to secure sufficient energy supply to meet forecasted demand plus a reserve margin as required by CPUC and CAISO."
                }
            }
            h2 {
                class: "section_title",
                "Responsibilities and Roles of CCAs"
            }
            ul {
                class: "list",
                li {
                    "Procurement of Generation Services: Procure the generation portion of electricity, offering competitive rates and energy options reflecting local preferences."
                }
                li {
                    "Scheduling and Settlement with CAISO: Coordinate with CAISO to ensure electricity delivery and account for power needs."
                }
                li {
                    "Customer Communications and Management: Handle customer inquiries, manage accounts, and provide service information."
                }
                li {
                    "Oversight of CCA Service Programs: Develop strategies for energy procurement, rate setting, customer engagement, and sustainability initiatives."
                }
            }
            h2 {
                class: "section_title",
                "CCA Service Offerings and Enrollment"
            }
            ul {
                class: "list",
                li {
                    "Service Offerings to Residential and Non-Residential Customers: All residents within a CCA service area are offered CCA service. CCAs may also serve businesses and other non-residential entities."
                }
                li {
                    "Opt-Out Enrollment Policy: Customers are automatically enrolled but can opt out by submitting a request. Clear instructions and deadlines are provided by the CCA."
                }
            }
            p {
                class: "paragraph",
                "CCA programs provide an alternative to traditional utility services, allowing local governments to procure power on behalf of their communities. The structure of CCA service offerings and the opt-out enrollment policy promote community engagement, increase access to renewable energy options, and offer potential economic benefits, empowering communities to innovate and support sustainability goals."
            }
        }
    }
}
