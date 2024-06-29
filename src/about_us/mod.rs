use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {}
}

#[component]
pub fn AboutUs() -> Element {
    rsx! {
       style { { include_str!("../css_files/about_us.css") } }
       div {class: "about_container",
            h1 { "About Us" }
            p { strong { "Tesero Sol" } }
            p { em { "June 21, 2024" } }

            h2 { "Executive Summary" }
            p { "Founded in 2024, Tesero Sol is a leader in renewable energy, focused on Southern California's Latino communities. Headquartered in Nevada, we specialize in solar energy generation, energy storage, EV charging solutions, and co-generation technologies. Our mission is to drive a decarbonized and electrified future through innovative and sustainable energy solutions tailored to Southern California’s diverse landscape." }

            h2 { "Our Commitment" }
            p { "Tesero Sol aims to be a trusted partner in providing clean, affordable, and reliable energy solutions. We support Latino businesses and municipalities in meeting their energy needs and advancing environmental sustainability and energy independence." }


        }
        TeamContainer {}
    }
}

#[component]
pub fn TeamContainer() -> Element {
    rsx! {
        div {
            class: "team-container",
            h2 { "Executive Team" }
            div {
                class: "row",
                TeamMember {
                    img_url: "https://picsum.photos/130/130?image=1027",
                    name: "Philip Skipitaris",
                    title: "Partner",
                },
                TeamMember {
                    img_url: "https://picsum.photos/130/130?image=839",
                    name: "Dr. William Francis",
                    title: "Partner",
                },
                TeamMember {
                    img_url: "https://picsum.photos/130/130?image=856",
                    name: "Peter Ryan",
                    title: "Partner",
                },
                TeamMember {
                    img_url: "https://picsum.photos/130/130?image=836",
                    name: "Racquel Cortez",
                    title: "Partner",
                },
            }
        }
    }
}

#[component]
pub fn TeamMember(img_url: String, name: String, title: String) -> Element {
    rsx! {
        div {
            class: "col-12 col-sm-6 col-md-4 col-lg-3",
            div {
                class: "our-team",
                div {
                    class: "picture",
                    img {
                        class: "img-fluid",
                        src: "{img_url}"
                    }
                },
                div {
                    class: "team-content",
                    h3 {
                        class: "name",
                        "{name}"
                    },
                    h4 {
                        class: "title",
                        "{title}"
                    }
                    p{ "Dr. William Francis oversees financial consultation, product development, and procurement"
                    }
                },
                ul {
                    class: "social",
                    li { a { href: "https://codepen.io/collection/XdWJOQ/", class: "fa fa-facebook", aria_hidden: "true" } },
                    li { a { href: "https://codepen.io/collection/XdWJOQ/", class: "fa fa-twitter", aria_hidden: "true" } },
                    li { a { href: "https://codepen.io/collection/XdWJOQ/", class: "fa fa-google-plus", aria_hidden: "true" } },
                    li { a { href: "https://codepen.io/collection/XdWJOQ/", class: "fa fa-linkedin", aria_hidden: "true" } }
                }
            }
        }
    }
}
