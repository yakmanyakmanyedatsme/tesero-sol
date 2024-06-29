use dioxus::prelude::*;

#[component]
pub fn OffRoadRebates() -> Element {
    rsx! {
        style { {include_str!("../css_files/off_road_rebates.css") }}
        div {
            class: "chdv_section",
            h2 {
                class: "section_title",
                "Clean Heavy-Duty Vehicles (CHDV) Grants"
            }
            p {
                class: "paragraph-main",
                "Welcome to the Clean Heavy-Duty Vehicles (CHDV) Grants information page!
                This program offers grants and incentives designed to help replace heavy-duty 
                vehicles with cleaner, more efficient alternatives. By supporting the transition to electric, 
                hybrid, and other low-emission technologies, the CHDV program aims to reduce emissions and 
                improve air quality across our communities."
            }
            h3 {
                class: "subsection_title",
                "Key Features of the CHDV Grants Program:"
            }
            ul {
                class: "list",
                li {
                    strong { "Eligibility: " }
                    "Municipalities, businesses, and other entities operating heavy-duty vehicles."
                    " Eligible vehicles must meet specific criteria related to age, mileage, and
                    emissions to qualify for the program."
                }
                li {
                    strong { "Funding: " }
                    "Grants typically cover a significant portion of the cost associated with purchasing new vehicles or retrofitting existing ones to meet cleaner standards."
                    " Additional funds may also be available for necessary infrastructure improvements, such as the installation of electric vehicle charging stations."
                }
                li {
                    strong { "Application Process: " }
                    "Applications must be submitted through the designated state or federal portal."
                    " Applicants will need to provide detailed information about their current vehicles, the intended use of the new or retrofitted vehicles, and the expected environmental benefits of the proposed changes."
                }
                li {
                    strong { "Selection Criteria: " }
                    "Projects are evaluated based on their potential to significantly reduce emissions."
                    " The cost-effectiveness of the project is a key consideration."
                    " The feasibility and readiness of the proposed project are crucial factors in the selection process."
                }
                li {
                    strong { "Reporting and Compliance: " }
                    "Grant recipients must provide regular reports on the performance and usage of the funded vehicles."
                    " Recipients must comply with all relevant federal and state regulations throughout the duration of the grant."
                }
            }
            h2 {
                class: "section_title",
                "Apply for a CHDV Grant Today!"
            }
            p {
                class: "paragraph",
                "The Clean Heavy-Duty Vehicles Grants program is your opportunity to contribute to a cleaner, healthier environment. Whether you're a municipality looking to upgrade your fleet or a business aiming to reduce your carbon footprint, this program offers the support you need to make a meaningful impact."
            }
            p {
                class: "paragraph",
                "For more information on current funding opportunities, application deadlines, and specific eligibility criteria, please visit our application portal or contact our support team."
            }
            p {
                class: "paragraph",
                "Ready to make a change? Apply now and drive the future of clean transportation!"
            }
        }
    }
}
