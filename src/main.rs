#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use tesero_sol::App;

fn main() {
    let cfg = server_only!(
        dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)))
    );
    LaunchBuilder::fullstack().with_cfg(cfg).launch(App);
}
