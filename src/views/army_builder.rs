use dioxus::prelude::*;

use crate::api::types::Metadata;

const ARMY_LIST_CSS: Asset = asset!("/assets/styling/army_list.css");

#[component]
pub fn ArmyBuilder() -> Element {
    let metadata = use_server_future(fetch_game_metadata)?;
    match &*metadata.read_unchecked() {
        Some(Ok(metadata)) => {
            use_context_provider(|| metadata.clone());
            let selected_faction: Signal<Option<u64>> = use_signal(|| None);
            rsx! {
                document::Link { rel: "stylesheet", href: ARMY_LIST_CSS }
                div { id: "screen",
                    super::factions_list::FactionsList { selected_faction }
                    match *selected_faction.read() {
                        Some(fac) => rsx! {
                            super::units_list::UnitsList { selected_faction: fac }
                            super::army_list::ArmyList { selected_faction: fac }
                        },
                        None => rsx! {
                            div {}
                        },
                    }
                }
            }
        }
        Some(Err(err)) => rsx! { "Error : {err:?}" },
        None => rsx! { "Ongoing" },
    }
}

#[server]
async fn fetch_game_metadata() -> Result<Metadata, ServerFnError> {
    crate::api::requests::fetch_metadata()
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e:?}")))
}
