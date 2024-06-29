pub mod dashboard;
pub mod orders;
use crate::dashboard::dashboard::dashboard::EnergyDashboard;
use crate::dashboard::dashboard::orders::DashboardOrders;
use dioxus::prelude::*;
use dioxus_free_icons::icons::*;
use dioxus_free_icons::Icon;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        style {{include_str!("../css_files/dashboard.css")}}
        div {
            class: "main",
        div {
            class: "sidebar",
            div {
                class: "top",

                div {
                    class:"logo",
                    img {
                        src: "https://i.ibb.co/1mgsPGY/tesero-sol-logo.png",
                        alt: "Logo React Europe",
                        title: "React Europe",
                        style: "width: 47%",
                    }
                }
                div {
                    class: "user",
                        p {"William Francis"}
                }
                ul {
                      li { style: "opacity: 0;",
                        Icon {
                                width: 30,
                                height: 25,
                                fill: "black",
                                icon: fa_solid_icons::FaBuilding,

                        }
                    },
                    li { a { href: "#",
                            Icon {
                                width: 30,
                                height: 25,
                                fill: "black",
                                icon: fa_solid_icons::FaBatteryFull,
                            }
                            span {class: "nav-item", "Dashboard" }
                        }
                        span { class: "tooltip", "Dashboard" }
                    },
                    li { a { href: "#",
                        Icon {
                                width: 30,
                                height: 25,
                                fill: "black",
                                icon: fa_solid_icons::FaBoxesPacking,
                            }
                            span {class: "nav-item", "Orders" }
                        }
                        span { class: "tooltip", "Orders" }
                    },
                    li { a { href: "#",
                        Icon {
                                width: 30,
                                height: 25,
                                fill: "black",
                                icon: fa_solid_icons::FaUser,
                            }
                            span {class: "nav-item",  "Account" }
                        }
                        span {class: "tooltip", "Account" }
                    },
                    li { a { href: "#",
                        Icon {
                                width: 30,
                                height: 25,
                                fill: "black",
                                icon: fa_solid_icons::FaCalendar,
                            }
                            span {class: "nav-item", "Calendar" }
                        }
                        span { class: "tooltip", "Calendar" }
                    },
                    li { style: "opacity: 0;",
                        Icon {
                                width: 30,
                                height: 25,
                                fill: "black",
                                icon: fa_solid_icons::FaBuilding,

                        }
                    }

                }
            }
        }
        div {
            class: "main-content",
            EnergyDashboard {}
        }
    }
    }
}
