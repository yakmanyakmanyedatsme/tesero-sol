#![allow(non_snake_case)]
pub mod about_us;
pub mod customers;
pub mod dashboard;
pub mod general;
pub mod homepage;
pub mod login;
pub mod municipal;
pub mod programs;
pub mod projects;
pub mod services;
pub mod store;
pub mod surreal_db_helper;
use crate::about_us::{AboutUs, Projects};
use crate::dashboard::dashboard::Dashboard;
use crate::homepage::*;
use crate::login::Login;
use crate::login::SignUp;
use crate::municipal::MunicipalProgram;
use crate::municipal::Ppa;
use crate::municipal::PublicPrivatePartnership;
use crate::programs::{residential::ResidentialProgram, Programs};
use crate::projects::adelanto::Adelanto;
use crate::projects::san_benito::SanBenito;
use crate::services::{application::Application, batteries::BatteryService, Services};
use crate::store::Store;
use dioxus::prelude::*;
/// An enum of all of the possible routes in the app.
#[rustfmt::skip]
#[derive(Routable, PartialEq, Clone)]
enum Route {
    // The home page is at the / route
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[end_layout]
    #[layout(NavBar)]
    #[route("/my-account")]
    Dashboard {},
    #[end_layout]
    #[route("/sign-up")]
    SignUp {},
    #[route("/login")]
    Login {},
    #[layout(NavBar)]
    #[route("/services")]
    Services {},
    #[end_layout]
    #[layout(NavBar)]
    #[route("/store")]
    Store {},
    #[end_layout]
    #[layout(NavBar)]
    #[route("/program/")]
    Programs{},
    #[end_layout]
    #[route("/program/battery-program")]
    BatteryService {},
    #[route("/services/municipal-offerings/power-purchase-agreement")]
    Ppa {},
    #[route("/municipal-offerings/public-private-partnership")]
    PublicPrivatePartnership {},
    #[layout(NavBar)]
    #[route("/municipal-offerings")]
    MunicipalProgram {},
    #[end_layout]
    #[route("/projects")]
    Projects {},
    #[route("/projects/adelanto")]
    Adelanto {},
    #[route("/projects/san-benito")]
    SanBenito {},
    #[layout(NavBar)]
    #[route("/about-us")]
    AboutUs {},
    #[end_layout]
    #[route("/programs/application")]
    Application {},
    #[layout(NavBar)]
    #[route("/programs/residential")]
    ResidentialProgram {},

}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        style { {include_str!("./css_files/nav_bar.css")}}
            body {
            nav { id: "main-nav",
                Link {id: "nav-link", to: Route::Services {}, "Services"} //a { class: "et-hero-tab", href: "#tab-es6", "Services"}
                Link {id: "nav-link", to: Route::MunicipalProgram {}, "Municipal Offerings"} //a { class: "et-hero-tab", href: "#tab-flexbox", "Municipal Programs" }
                Link {id: "nav-link", to: Route::Projects {}, "Projects"} //a { class: "et-hero-tab", href: "#tab-react", "Partners" }
                //Link {id: "nav-link", to: Route:: Store {}, "Store"} //a { class: "et-hero-tab", href: "#tab-angular", "About Us" }
                Link {id: "nav-link", to: Route::Programs{}, "Programs"} //a { class: "et-hero-tab", href: "#tab-angular", "About Us" }
                Link {id: "nav-link", to: Route::AboutUs {}, "About Us"} //a { class: "et-hero-tab", href: "#tab-angular", "About Us" }
                div {id: "right-tab",
                    Link {id: "button-is-primary",to: Route::Login { }, "Login"} // a { class: "button is-ghost", href: "#login", "Log In" }
                    Link {id: "button-is-ghost",to: Route::SignUp {  }, "Sign Up"}//a { class: "button is-primary", href: "#signup", "Sign Up" }
                }
            }
            Outlet::<Route> { }
        }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> { }
    }
}
