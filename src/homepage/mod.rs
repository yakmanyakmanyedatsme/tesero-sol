use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
           style { {include_str!("homepage_style.css")}}
           div { class: "outer_div",
           div { class: "banner",
           div { class: "intro",
                   img {
                       src: "https://i.ibb.co/1mgsPGY/tesero-sol-logo.png",
                       alt: "Logo React Europe",
                       title: "React Europe",
                   }
               }
               div {
                   class: "top_header",
                   p { "In collaboration with our SoCal based partners we provide Southern
                     California cities and residents with top-tier installation and maintenance
                      services for energy systems.  " }
               }
               div { class: "block_image",
               h2 {"We assists municipalities and their residents cut energy expenses"}
               img {
                   src: "https://i.ibb.co/MGKhmkV/Group-1-1.png",
                   alt: "Group-1",
                   style: "border: 0;  margin-left: 10%; margin-right: 10%; width: 65vh;",
               }
           }
           h2 {class: "main_header2","How We Work"}
            section {
           class: "image-container",
           div {
           img {
               src: "https://i.ibb.co/nk8L2dw/Group-1-6.png",
               alt: "Group-1-4",
               style: "border: 0; width: 95%; height: 90%;",
           }
       }
       div{
           section {
           class: "energy-services-section",
           h1 { "Discover Energy Efficiency with Tesero Sol" },
           p {
               "At Tesero Sol, we specialize in providing comprehensive energy solutions tailored to your specific needs. We're here to help you optimize your energy usage, reduce costs, and enhance operational efficiency with our end-to-end services."
           },
           h1 { "Process:" },
           ol {
               li { "Submit Your Utility Bills: Start your journey to energy efficiency by providing us with your most recent energy bills. This allows us to analyze your current energy usage and identify key areas for improvement." },
               li { "Receive Professional Recommendations: Based on our analysis, our team of energy experts will devise a personalized plan to increase your energy efficiency. Our suggestions are designed to integrate seamlessly with the latest technology and sustainable practices." },
               li { "Installation by Experts: Once you approve our recommendations, our skilled technicians will take care of the installation process. We ensure that all systems are set up with precision and care, minimizing any disruption to your daily operations." },
               li { "Operational Support: Post-installation, we don’t just walk away. We provide ongoing operational support to ensure that your new energy systems perform optimally, helping you save energy and costs without any extra effort on your part." },
           },
           h3 { "Ready to Reduce Your Energy Costs?" }
       }
       }
       }
               div { class: "big_container",
                   div { class: "flex_item",
                       h1 {"Municipal Programs"}
                       p {"Municipal energy services refer to the energy-related services
                         provided at the city or municipal level, typically managed or 
                         coordinated by local government bodies or municipal-owned utilities.
                         These services are aimed at supporting public infrastructure, local
                          businesses, and residences within a municipality."}
                       div { class: "download_buttons",
                           a { id: "button", href: "#",
                               i { class: "ibutt_stuff" }
                               span { "NEVI" }
                           }
                           a { id: "button", href: "#",
                               i { class: "ibutt_stuff" }
                               span { "Transporation Center" }
                           }

                       }
                       div { class: "download_buttons",
                     a { id: "button", href: "#",
                               i { class: "ibutt_stuff" }
                               span { "CCA" }
                           }
                           a { id: "button", href: "#",
                               i { class: "ibutt_stuff" }
                               span { "Off Road Rebate" }
                           }
                           }
                       }

           div {class: "flex_item",
               h1 {"Business & Residential Programs"}
               p {"Residential energy services are focused on individual households,
                providing them with electricity, gas, and other energy-related services.
                 These services are typically provided by utility companies, which may 
                 be privately owned or government-owned. "}
               div { class: "download_buttons",

                   a { id: "button", href: "#",
                       i { class: "ibutt_stuff" }
                       span { "Battery-Storage" }
                   }
                   a { id: "button", href: "#",
                       i { class: "ibutt_stuff" }
                       span { "Solar" }
                   }
               }
               div { class: "download_buttons",
                   a { id: "button", href: "#",
                       i { class: "ibutt_stuff" }
                       span { "EV Charger" }
                   }
                   a { id: "button", href: "#",
                       i { class: "ibutt_stuff" }
                       span { "PPA" }
                   }
               }

           }
       }

       }
       img {
                   src: "https://i.ibb.co/QJ44wGD/Group-10.png",
                   style: "border: none; width: 75%; margin-left: 12.5%;", // Ensure there's no border as specified in the HTML
               }
       }
       div { class: "main_header",
           h1 {"Residential Offering"}
       }
        section {
           class: "grid-container-residential",
           div {
               class: "grid-item-img",
               img {
                   src: "https://i.ibb.co/v4jfNpX/image-2.png",
                   alt: "Batteries",
                   style: "border: 0;height: 25vh;"
               }
               h2 {"Batteries"}
           },
           div {
               class: "grid-item-img",
               img {
                   src: "https://i.ibb.co/QFqg1g4/image-4.png",
                   alt: "Solar",
                   style: "border: 0; height: 30vh;"
               }
               h2 {"Solar"}
           },
           div {
               class: "grid-item-img",
               img {
                   src: "https://i.ibb.co/Zc2YXG6/image-5.png",
                   alt: "EV Chargers",
                   style: "border: 0; height: 30vh;"
               }
               h2 { "EV Charger"}
           },
           div { class: "grid-item",
           "A residential solar system involves installing photovoltaic panels
            on a home to harness energy from the sun. This system reduces reliance
            on traditional power grids, lowers electricity bills, and decreases
            carbon emissions, making it an environmentally friendly option for 
            homeowners." 
            },
           div { class: "grid-item", "A home battery system stores electricity,
             either from the grid or solar panels, for later use. This is especially 
             useful during power outages or peak electricity pricing times, enhancing 
             energy independence and reducing costs." 
        },
           div { class: "grid-item", "An Electric Vehicle (EV) charger
             installed at home allows for convenient charging of electric
              vehicles. It increases the practicality of owning an EV by
               ensuring it is always ready for use, promoting a shift away 
               from fossil fuel-dependent transportation." 
           }
        }
       div{
       class: "microgrid_image_container",
       div {
           class: "microgrid_image",
           img {
               src: "https://i.ibb.co/7JWTDrj/Group-1-3.png",
               alt: "CA-Map",
               style: "border: 0; width: 75vh;",
           }
       },
       div {
           class: "microgrid_image",
           h2 {"Installation and system Operation Coverage Areas"}
           ul {
               li { "Adelanto" }
               li { "Norwalk" }
               li { "San Benito" }
           }
           }
       }

       div { class: "main_header",
           h1 {"Municipal Offering"}
       }
    section {
           class: "grid-container-municipal",
           div {
               class: "grid-item-img",
               img {
                   src: "https://i.ibb.co/SmcTpP8/PPAs.png",
                   alt: "PPAs",
                   style: "border: 0; height: 25vh;"
               }
               h2{"Power Purchase Agreement"}
           },
           div {
               class: "grid-item-img",
               img {
                   src: "https://i.ibb.co/VQvPHdq/Simulations.png",
                   alt: "Simulations",
                   style: "border: 0;height: 25vh;"
               }
               h2 {"Cost-Benefit Analysis"}
           },
           div {
               class: "grid-item-img",
               img {
                   src: "https://i.ibb.co/TMv3YV6/image-3.png",
                   alt: "Operating Systems",
                   style: "border: 0; height: 20vh;"
               }
               h2 {"Systems Operation & maintenance"}
           },
           div {
               class: "grid-item-img",
               img {
                   src: "https://i.ibb.co/VMrHkcd/hybrid-renewable-microgrid.png",
                   alt: "Hybrid Renewable Microgrid",
                   style: "border: 0; height: 25vh;"
               }
               h2 { "Microgrid"}
           },
           div { class: "grid-item",
           "Power Purchase Agreements (PPAs) as a service refer to a financial arrangement in which a third-party
            developer owns, operates, and maintains the photovoltaic (PV) system, and a host customer agrees to 
            site the system on their property and purchases the system's electric output from the solar services
             provider for a predetermined period."},
           div { class: "grid-item",
           "Through the use of simulation and other analysis we evaluate the cost and benefits
            of the energy instillation. Going through a variety of use cases and system specifications."
            },
           div { class: "grid-item",
           "This includes training personnel, implementing best practices, and optimizing performance.
           Additionally, we provide ongoing monitoring and troubleshooting to ensure optimal functionality."
            },
           div { class: "grid-item",
           "Energy microgrids are small-scale power grids that can operate independently or in conjunction with
            the area's main electrical grid. These systems are capable of balancing captive supply and demand,
            and they can operate \"off the grid\" during larger grid outages. Microgrids can power a single 
            facility or a larger area."
            }
    }
    section {
        div { class: "main_header",
            h1 {"Past Customers"}
        }
       div {
           class: "grid-container",
           figure { class: "grid-item", img { src: "image1.jpg", alt: "Image 1" }, figcaption { "CIOPROUSA" } },
           figure { class: "grid-item", img { src: "image2.jpg", alt: "Image 2" }, figcaption { "ASG" } },
           figure { class: "grid-item", img { src: "image3.jpg", alt: "Image 3" }, figcaption { "Greenrock" } },
        }
        div {
            class: "grid-container",
            figure { class: "grid-item", img { src: "image4.jpg", alt: "Image 4" }, figcaption { "Powin" } },
            figure { class: "grid-item", img { src: "image5.jpg", alt: "Image 5" }, figcaption { "Qpoe" } },
            figure { class: "grid-item", img { src: "image6.jpg", alt: "Image 6" }, figcaption { "Ilios Energy" }},
        }
    }
       } // Contact and support information
}
//"https://i.ibb.co/v4jfNpX/image-2.png"
//<img src=alt="image-3" border="0">
