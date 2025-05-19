use dioxus::prelude::*;

use closure::closure;

use crate::api::types::{Faction, Metadata};

const ARMY_LIST_CSS: Asset = asset!("/assets/styling/army_list.css");

fn is_high_level_faction(faction: &Faction) -> bool {
    faction.parent == faction.id
}

fn on_high_level_faction_clicked(faction: u64, mut unrolled_faction: Signal<Option<u64>>) {
    let current = *unrolled_faction.clone().read();
    match current {
        Some(f) => {
            if f == faction {
                unrolled_faction.set(None);
            } else {
                unrolled_faction.set(Some(faction));
            }
        }
        None => {
            unrolled_faction.set(Some(faction));
        }
    }
}

#[component]
pub fn ArmyList() -> Element {
    let metadata = use_server_future(echo_server)?;
    let unrolled_faction: Signal<Option<u64>> = use_signal(|| None);
    let mut selected_faction: Signal<Option<u64>> = use_signal(|| None);
    rsx! {
        document::Link { rel: "stylesheet", href: ARMY_LIST_CSS }
        div { id: "screen",
            div { id: "armies_list",
                match &*metadata.read_unchecked() {
                    Some(Ok(metadata)) => rsx! {
                        "Armies"
                        for faction in metadata.factions.iter().filter(|f| is_high_level_faction(f)) {
                            div {
                                class: "army_selection high_level",
                                onclick: closure!(
                                    clone faction, clone mut unrolled_faction, | _ | {
                                    on_high_level_faction_clicked(faction.id, unrolled_faction.clone()); }
                                ),
                                "{faction.name}"
                            }
                            if *unrolled_faction.read() == Some(faction.id) {
                                for faction in metadata.factions.iter().filter(|f| f.parent == faction.id) {
                                    div {
                                        class: "army_selection",
                                        onclick: closure!(clone faction, | _ | { * selected_faction.write() = Some(faction.id); }),
                                        "{faction.name}"
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                    "Error : {err:?}"
                    },
                    None => rsx! { "Ongoing" },
                }
            }
            div {
                match &*selected_faction.read() {
                    Some(faction_id) => {
                        rsx! {
                            div { "{faction_id}" }
                        }
                    }
                    None => rsx! { "Plop" },
                }
            }
        }
    }
}

#[server]
async fn echo_server() -> Result<Metadata, ServerFnError> {
    crate::api::requests::fetch_metadata()
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e:?}")))
}
