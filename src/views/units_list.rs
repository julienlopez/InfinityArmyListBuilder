use dioxus::prelude::*;

use crate::api::types::{Equipment, FactionData, Metadata, Profile, Resume, Skill, Unit, WikiItem};

#[component]
pub fn UnitsList(selected_faction: u64) -> Element {
    let faction_data = use_server_future(move || fetch_faction_data(selected_faction))?;
    rsx! {
        div { class: "faction_troop_selection",
            match &*faction_data.read_unchecked() {
                Some(Ok(fac)) => rsx! {
                    for (unit , resume) in std::iter::zip(&fac.units, &fac.resume) {
                        UnitBox { unit: unit.clone(), resume: resume.clone() }
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
fn UnitBox(unit: Unit, resume: Resume) -> Element {
    let mut is_deployed = use_signal(|| false);
    rsx! {
        div { onclick: move |_| is_deployed.toggle(),
            img { class: "unit_logo", src: "{resume.logo}" }
            span { class: "unit_name", "{unit.name}" }
        }
        if *is_deployed.read() {
            // TODO remove unwrap
            UnitDetails {
                profile: unit.profileGroups
                    .iter()
                    .nth(0)
                    .and_then(|profile_group| profile_group.profiles.iter().nth(0))
                    .unwrap()
                    .clone(),
            }
        }
    }
}

fn movement_string(profile: &Profile) -> String {
    profile
        .r#move
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<String>>()
        .join("-")
}

#[component]
fn UnitDetails(profile: Profile) -> Element {
    let move_c = movement_string(&profile);
    rsx! {
        div {
            table { class: "troop_details",
                tr {
                    th { "MOV" }
                    th { "CC" }
                    th { "BS" }
                    th { "PH" }
                    th { "WIP" }
                    th { "ARM" }
                    th { "BTS" }
                    th { "VITA" }
                    th { "S" }
                    th { "AVA" }
                }
                tr {
                    td { "{move_c}" }
                    td { "{profile.cc}" }
                    td { "{profile.bs}" }
                    td { "{profile.ph}" }
                    td { "{profile.wip}" }
                    td { "{profile.arm}" }
                    td { "{profile.bts}" }
                    td { "{profile.w}" }
                    td { "{profile.s}" }
                    td { "{profile.ava}" }
                }
            }
            EquipmentBox { equipment: profile.equip.clone() }
            SkillsBox { skills: profile.skills.clone() }
        }
    }
}

#[component]
fn EquipmentBox(equipment: Vec<Equipment>) -> Element {
    let metadata = consume_context::<Metadata>();
    rsx! {
        div {
            span { "Equipment:" }
            for e in equipment {
                WikiLinkLabel {
                    label: metadata
                        .equips
                        .iter()
                        .find_map(|eq| if eq.id == e.id { Some(eq.clone()) } else { None })
                        .unwrap(),
                }
            }
        }
    }
}

#[component]
fn WikiLinkLabel(label: WikiItem) -> Element {
    rsx! {
        if let Some(link) = label.wiki {
            Link { class: "wiki_label clickable", to: link, new_tab: true, "{label.name}" }
        } else {
            span { class: "wiki_label", "{label.name}" }
        }
    }
}

#[component]
fn SkillsBox(skills: Vec<Skill>) -> Element {
    let metadata = consume_context::<Metadata>();
    rsx! {
        div {
            span { "Skills:" }
            for e in skills {
                WikiLinkLabel {
                    label: metadata
                        .skills
                        .iter()
                        .find_map(|eq| if eq.id == e.id { Some(eq.clone()) } else { None })
                        .unwrap(),
                }
            }
        }
    }
}

#[server]
async fn fetch_faction_data(faction: u64) -> Result<FactionData, ServerFnError> {
    crate::api::requests::fetch_faction_data(faction)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("{e:?}")))
}
