use dioxus::prelude::*;
#[component]
pub fn Store() -> Element {
    rsx! {
        div {
            class: "snip1423",
            figure {
                img { src: "https://s3-us-west-2.amazonaws.com/s.cdpn.io/331810/sample57.jpg", alt: "sample57" },
                figcaption {
                    h3 { "Pudol Doux" }
                    p { "All this modern technology just makes people try to do everything at once." }
                    div { class: "price", s { "$24.00" } "$19.00" }
                }
                i { class: "ion-android-cart" }
                a { href: "#" }
            }
            figure {
                class: "snip1423 hover",
                img { src: "https://s3-us-west-2.amazonaws.com/s.cdpn.io/331810/sample98.jpg", alt: "sample98" },
                figcaption {
                    h3 { "Kurie Secco" }
                    p { "To make a bad day worse, spend it wishing for the impossible." }
                    div { class: "price", s { "$29.00" } "$39.00" }
                }
                i { class: "ion-android-cart" }
                a { href: "#" }
            }
            figure {
                class: "snip1423",
                img { src: "https://s3-us-west-2.amazonaws.com/s.cdpn.io/331810/sample90.jpg", alt: "sample90" },
                figcaption {
                    h3 { "Zosaisan Invec" }
                    p { "Why should I have to work for everything? It's like saying that I don't deserve it." }
                    div { class: "price", s { "$35.00" } "$45.00" }
                }
                i { class: "ion-android-cart" }
                a { href: "#" }
            }
        }
    }
}
