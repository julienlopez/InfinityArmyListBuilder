use dioxus::prelude::*;

use crate::api::types::Metadata;

const ARMY_LIST_CSS: Asset = asset!("/assets/styling/army_list.css");

#[component]
pub fn ArmyList() -> Element {
    let metadata = use_server_future(echo_server)?;
    rsx! {
        document::Link { rel: "stylesheet", href: ARMY_LIST_CSS }
        div { id: "screen",
            div { id: "armies_list",
                match &*metadata.read_unchecked() {
                    Some(Ok(metadata)) => rsx! {
                        "Armies"
                        for faction in &metadata.factions {
                            div { "{faction.name}" }
                        }
                    },
                    Some(Err(err)) => rsx! {
                    "Error : {err:?}"
                    },
                    None => rsx! { "Ongoing" },
                }
            }
            div { "Plop" }
        }
    }
}

#[server]
async fn echo_server() -> Result<Metadata, ServerFnError> {
    crate::api::requests::fetch_metadata()
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e:?}")))
}
