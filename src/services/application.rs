use dioxus::prelude::*;

#[component]
pub fn Application() -> Element {
    rsx! {
        style { {include_str!("apply_style.css")} }

        body {
        section { class: "agile-volt",
            div { class: "agile-voltheader",
                h1 { "Battery Program Application" }
            }
            div { class: "agile-voltsub",
                h2 { "Application Form" }
                p { style: "font-size: 1.5em; text-align: center;", "fill in the following fields to apply for the program"}
                div {style: "height: 3vh;",}
                form { action: "#", method: "post",
                    div { class: "agile-password",
                        input { r#type: "text", name: "username", placeholder: "Name", required: "true", autofocus: "true" }
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "email", name: "email", placeholder: "Email", required: "true" }
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "address", placeholder: "Address", required: "true" }
                        div { class: "line",}
                    }
                    p { style: "font-size:2.5rem; text-decoration: underline; font-weight: 1000", "Create Password :" }
                    div { class: "agile-password",
                        input { style: "height: 3vh; margin-top: 1vh;", r#type: "password", name: "address", placeholder: "new password", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { style: "height: 3vh; margin-top: 1vh;", r#type: "password", name: "address", placeholder: "re-enter password", required: "true"}
                        div { class: "line",}
                    }

                    div {
                        p { "City" }
                        select {style: "background-color:#595959; border: 2px solid #5acae9;",
                            option { style: "font-size: 2.5rem;", value: "county", "" }
                            option { style: "font-size: 2.5rem;",value: "county", "Norwalk" }
                            option { style: "font-size: 2.5rem;",value: "county", "San Benito" }
                        }
                    }
                    div { class: "clear" }
                    div { class: "agile-password",
                        input { r#type: "tel", name: "zipcode", placeholder: "Zip Code" }
                        div { class: "line",}
                    }
                    div { class: "clear" }
                    div { class: "agile-password",
                        input { r#type: "text", name: "phonenumber", placeholder: "Phone Number", required: "true", autofocus: "true" }
                        div { class: "line",}
                    }
                    div { class: "clear" }
                    input {class: "submit-button", r#type: "submit", value: "submit" }
                }
            }
        }
        div { style: "height: 3%;" }
        footer{
            a { href: "https://betterdiscord.net",
                div { class: "bottom-wrapper",
                    div { class: "wsite", "Official BetterDiscord Website" }
                }
            }
        }
    }
    }
}
