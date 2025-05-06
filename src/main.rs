#![allow(non_snake_case)]

mod api;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

static CSS: Asset = asset!("/assets/main.css");

fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let metadata = use_resource(|| async move { crate::api::requests::fetch_metadata().await });

    rsx! {
        div {
            id: "header",
            "Infinity Army List Builder"
        }
        div {
            id: "screen",
            div {
                id: "armies_list",
                "Armies"
                // tracing::logger::info!("{armies}");
                match &*metadata.read_unchecked() {
                    Some(Ok(metadata)) =>
                    {
                        rsx!{
                        for faction in &metadata.factions {
                            div {
                                "Plop"
                                "{faction.name}"
                            }}
                        }
                    },
                    Some(Err(_)) =>
                    {
                        info!("plop");
                        rsx! {
                        div { "Loading dogs failed" }
                    }},
                    None => rsx! {
                        div { "Loading dogs..." }
                    },
                }
            }
            div {
                "Plop"
            }
        }
    }
}
