use crate::customers::individuals::Individual;
use crate::general::address::Address;
use crate::general::contactinfo::ContactInfo;
use crate::general::credentials::Credentials;
use crate::surreal_db_helper::*;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn SignUp() -> Element {
    let mut values = use_signal(HashMap::<String, FormValue>::new);
    let mut submitted_values = use_signal(HashMap::<String, FormValue>::new);
    let mut new_user = use_signal(Individual::default);
    let db = use_resource(use_reactive(&new_user, |new_user| async move {
        connect_and_signup_db("https://tesero-sol-db.fly.dev/rpc", "root", "root",new_user()).await
    }));
    rsx! {
        style { {include_str!("login_style.css")} }
            body {
            section { class: "agile-volt",
                div { class: "agile-voltheader",
                    h1 { "Battery Program Application" }
                }
                div { class: "agile-voltsub",
                h2 { "Application Form" }
                p { style: "font-size: 1.5em; text-align: center;", "fill in the following fields to apply for the program"}
                form {
                    // You can attach a handler to the entire form
                    oninput: move |ev| {
                        values.set(ev.values());
                    },
                    onsubmit: move |ev| async move {
                        let cred = Credentials {
                            email: ev.values().get("email").unwrap().0[0].clone(),
                            password: ev.values().get("password").unwrap().0[0].clone(),
                        };
                        let address = Address {
                            street_address: ev.values().get("address").unwrap().0[0].clone(),
                            city: ev.values().get("city").unwrap().0[0].clone(),
                            state_province: "California".to_string(),
                            zip_postal_code: ev.values().get("zipcode").unwrap().0[0].clone(),
                            country: "United States".to_string(),
                        };
                        let contact = ContactInfo {
                            first:ev.values().get("firstname").unwrap().0[0].clone(),
                            last:ev.values().get("lastname").unwrap().0[0].clone(),
                            middle_name:Some(ev.values().get("middlename").unwrap().0[0].clone()),
                            phone_number: ev.values().get("phonenumber").unwrap().0[0].clone(),
                            email: ev.values().get("email").unwrap().0[0].clone(),
                        };
                        new_user.set(Individual::new(cred,contact,address));
                        submitted_values.set(ev.values());
                    },
                    div { class: "agile-password",
                        input { r#type: "text", name: "firstname", placeholder: "First Name", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "lastname", placeholder: "Last Name", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "middlename", placeholder: "Middle Name", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "phonenumber", placeholder: "Phone Number", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "email", placeholder: "Email", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "address", placeholder: "Address", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "zipcode", placeholder: "Zip Code", required: "true"}
                        div { class: "line",}
                    }
                    div {
                        p { "City" }
                        select {
                        name: "city",
                        multiple: true,
                        oninput: move |ev| {
                            println!("Input event: {:#?}", ev);
                            println!("Values: {:#?}", ev.value().split(',').collect::<Vec<_>>());
                        },
                            option { style: "font-size: 2.5rem;", value: "empty", "" }
                            option { style: "font-size: 2.5rem;",value: "Norwalk", "Norwalk" }
                            option { style: "font-size: 2.5rem;",value: "San Benito", "San Benito" }
                        }
                        }
                        div { class: "clear" }
                        div { class: "agile-password",
                            input { style: "height: 3vh; margin-top: 1vh;", r#type: "text", name: "password",
                            placeholder: "new password", required: "true"}
                            div { class: "line",}
                        }
                        div { class: "agile-password",
                            input { style: "height: 3vh; margin-top: 1vh;", r#type: "text",
                            name: "re_enter_password", placeholder: "re-enter password", required: "true"}
                            div { class: "line",}
                        }
                        div { class: "clear" }
                    // Buttons will submit your form by default.
                    button { class: "submit-button",r#type: "submit", value: "Submit", "Submit the form" }
                    }
                }
            div { style: "height: 3%;" }
            if !submitted_values.read().is_empty() {
                    h2 { "Submitted! ✅" }
                    pre { "{db.read():#?}" }
                }
            div {style: "height: 3vh;",}

            }

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


#[component]
pub fn Login() -> Element {
    let mut values = use_signal(HashMap::<String, FormValue>::new);
    let mut submitted_values = use_signal(HashMap::<String, FormValue>::new);

    rsx! {
        style { {include_str!("login_style.css")} }
            body {
            section { class: "agile-volt",
                div { class: "agile-voltheader",
                    h1 { "Battery Program Application" }
                }
                div { class: "agile-voltsub",
                h2 { "Application Form" }
                p { style: "font-size: 1.5em; text-align: center;", "fill in the following fields to apply for the program"}
                form {
                    // You can attach a handler to the entire form
                    oninput: move |ev| {
                        values.set(ev.values());
                    },
                    onsubmit: move |ev| async move {
                        let cred = Credentials {
                            email: ev.values().get("email").unwrap().0[0].clone(),
                            password: ev.values().get("password").unwrap().0[0].clone(),
                        };
                        let _ = connect_and_signin_db("https://tesero-sol-db.fly.dev/rpc", "root", "root",cred).await;
                        submitted_values.set(ev.values());
                    },
                    div { class: "agile-password",
                        input { r#type: "text", name: "firstname", placeholder: "First Name", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "lastname", placeholder: "Last Name", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "middlename", placeholder: "Middle Name", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "phonenumber", placeholder: "Phone Number", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "email", placeholder: "Email", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "address", placeholder: "Address", required: "true"}
                        div { class: "line",}
                    }
                    div { class: "agile-password",
                        input { r#type: "text", name: "zipcode", placeholder: "Zip Code", required: "true"}
                        div { class: "line",}
                    }
                    div {
                        p { "City" }
                        select {
                        name: "city",
                        multiple: true,
                        oninput: move |ev| {
                            println!("Input event: {:#?}", ev);
                            println!("Values: {:#?}", ev.value().split(',').collect::<Vec<_>>());
                        },
                            option { style: "font-size: 2.5rem;", value: "empty", "" }
                            option { style: "font-size: 2.5rem;",value: "Norwalk", "Norwalk" }
                            option { style: "font-size: 2.5rem;",value: "San Benito", "San Benito" }
                        }
                        }
                        div { class: "clear" }
                        div { class: "agile-password",
                            input { style: "height: 3vh; margin-top: 1vh;", r#type: "text", name: "password",
                            placeholder: "new password", required: "true"}
                            div { class: "line",}
                        }
                        div { class: "agile-password",
                            input { style: "height: 3vh; margin-top: 1vh;", r#type: "text",
                            name: "re_enter_password", placeholder: "re-enter password", required: "true"}
                            div { class: "line",}
                        }
                        div { class: "clear" }
                    // Buttons will submit your form by default.
                    button { class: "submit-button",r#type: "submit", value: "Submit", "Submit the form" }
                    }
                }
            div { style: "height: 3%;" }
            if !submitted_values.read().is_empty() {
                    h2 { "Submitted! ✅" }
                    //pre { "{db.read():#?}" }
                }
            div {style: "height: 3vh;",}

            }

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