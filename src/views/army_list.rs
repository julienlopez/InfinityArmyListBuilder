use dioxus::prelude::*;

use closure::closure;

use crate::api::types::{Faction, FactionData, Metadata};

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
fn FactionList(
    metadata: Resource<Result<Metadata, ServerFnError>>,
    unrolled_faction: Signal<Option<u64>>,
    selected_faction: Signal<Option<u64>>,
) -> Element {
    rsx! {
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
    }
}

#[component]
fn TroopList(selected_faction: u64) -> Element {
    let faction_data = use_server_future(move || fetch_faction_data(selected_faction))?;
    rsx! {
        div { class: "faction_troop_selection",
            match &*faction_data.read_unchecked() {
                Some(Ok(fac)) => rsx! {
                    for (unit , resume) in std::iter::zip(&fac.units, &fac.resume) {
                        div {
                            img { class: "unit_logo", src: "{resume.logo}" }
                            span { class: "unit_name", "{unit.name}" }
                        }
                    }
                },
                Some(Err(err)) => rsx! {
                "Error : {err:?}"
                },
                None => rsx! { "Ongoing" },
            }
        }
    }
}

#[component]
fn CurrentArmyList() -> Element {
    rsx! {
        div { class: "current_army_list" }
    }
}

#[component]
pub fn ArmyList() -> Element {
    let metadata = use_server_future(fetch_game_metadata)?;
    let unrolled_faction: Signal<Option<u64>> = use_signal(|| None);
    let selected_faction: Signal<Option<u64>> = use_signal(|| None);
    rsx! {
        document::Link { rel: "stylesheet", href: ARMY_LIST_CSS }
        div { id: "screen",
            FactionList { metadata, unrolled_faction, selected_faction }
            match *selected_faction.read() {
                Some(fac) => rsx! {
                    TroopList { selected_faction: fac }
                },
                None => rsx! {
                    div {}
                },
            }
            CurrentArmyList {}
        }
    }
}

#[server]
async fn fetch_game_metadata() -> Result<Metadata, ServerFnError> {
    crate::api::requests::fetch_metadata()
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e:?}")))
}

#[server]
async fn fetch_faction_data(faction: u64) -> Result<FactionData, ServerFnError> {
    crate::api::requests::fetch_faction_data(faction)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e:?}")))
}
