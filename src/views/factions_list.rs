use dioxus::prelude::*;

use closure::closure;

use crate::api::types::{Faction, Metadata};

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
pub fn FactionsList(selected_faction: Signal<Option<u64>>) -> Element {
    let metadata = consume_context::<Metadata>();
    let unrolled_faction: Signal<Option<u64>> = use_signal(|| None);
    rsx! {
        div { id: "armies_list",
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
        }
    }
}
