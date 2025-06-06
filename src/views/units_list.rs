use dioxus::prelude::*;

use crate::api::types::{
    Equipment, FactionData, Metadata, Profile, ProfileGroup, Resume, Skill, Unit, UnitOption,
    Weapon, WeaponRef, WikiItem,
};

fn unit_type(unit: &Unit) -> Option<u8> {
    unit.profileGroups
        .iter()
        .nth(0)
        .and_then(|p| p.profiles.iter().nth(0))
        .map(|p| p.r#type)
}

fn sort_units<'a>(mut units: Vec<(&'a Unit, &'a Resume)>) -> Vec<(&'a Unit, &'a Resume)> {
    units.sort_by(|(u1, _), (u2, _)| unit_type(u1).cmp(&unit_type(u2)));
    units
}

#[component]
pub fn UnitsList(selected_faction: u64) -> Element {
    let faction_data = use_server_future(move || fetch_faction_data(selected_faction))?;
    rsx! {
        div { class: "faction_troop_selection",
            match &*faction_data.read_unchecked() {
                Some(Ok(fac)) => rsx! {
                    for (unit , resume) in sort_units(
                        std::iter::zip(&fac.units, &fac.resume)
                            .filter(|(u, _)| u.factions.contains(&selected_faction))
                            .collect(),
                    )
                    {
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

fn unit_type_string(r#type: u8) -> &'static str {
    if r#type == 1 {
        "LI"
    } else if r#type == 2 {
        "MI"
    } else if r#type == 3 {
        "HI"
    } else if r#type == 4 {
        "TAG"
    } else if r#type == 5 {
        "REM"
    } else if r#type == 6 {
        "SK"
    } else if r#type == 7 {
        "WB"
    } else {
        "??"
    }
}

#[component]
fn UnitBox(unit: Unit, resume: Resume) -> Element {
    let mut is_deployed = use_signal(|| false);
    // TODO remove unwraps
    let unit_profile_group = unit.profileGroups.iter().nth(0).unwrap();
    let unit_profile = unit_profile_group.profiles.iter().nth(0).unwrap();
    rsx! {
        div { class: "unit_box",
            div { class: "unit_header", onclick: move |_| is_deployed.toggle(),
                img { class: "unit_logo", src: "{resume.logo}" }
                span { class: "unit_name", "{unit.name}" }
                span { class: "unit_type", "{unit_type_string(unit_profile.r#type)}" }
            }
            if *is_deployed.read() {
                UnitDetails { profile_group: unit_profile_group.clone() }
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
fn UnitDetails(profile_group: ProfileGroup) -> Element {
    let profile = profile_group.profiles.iter().nth(0).unwrap();
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
            OptionsBox { options: profile_group.options.clone() }
        }
    }
}

fn generate_weapons_string(metadata: &Vec<Weapon>, weapons: &Vec<WeaponRef>) -> String {
    itertools::join(
        itertools::sorted(weapons).map(|w| {
            metadata
                .iter()
                .find(|wmd| Some(wmd.id) == w.id)
                .map(|wmd| wmd.name.clone())
                .unwrap_or("ERROR".to_string())
        }),
        ", ",
    )
}

static ORDERS_ICONS: phf::Map<&str, Asset> = phf::phf_map! {
    "REGULAR" => asset!("assets/images/orders/regular.png",
        ImageAssetOptions::new().with_size(ImageSize::Manual {width: 24,height: 24})),
    "LIEUTENANT" => asset!("assets/images/orders/lieutenant.png",
        ImageAssetOptions::new().with_size(ImageSize::Manual {width: 24,height: 24})),
    "IRREGULAR" => asset!("assets/images/orders/irregular.png",
        ImageAssetOptions::new().with_size(ImageSize::Manual {width: 24,height: 24})),
    "IMPETUOUS" => asset!("assets/images/orders/impetuous.png",
        ImageAssetOptions::new().with_size(ImageSize::Manual {width: 24,height: 24})),
    "TACTICAL" => asset!("assets/images/orders/tactical.png",
        ImageAssetOptions::new().with_size(ImageSize::Manual {width: 24,height: 24}))
};

#[component]
fn OptionsBox(options: Vec<UnitOption>) -> Element {
    let metadata = consume_context::<Metadata>();
    rsx! {
        div {
            table { class: "unit_options_table",
                tr {
                    th {}
                    th { "Name" }
                    th { "SWC" }
                    th { "PTS" }
                }
                for option in itertools::sorted(options) {
                    tr {
                        td { class: "unit_order_box",
                            for order in option.orders {
                                for o in itertools::repeat_n(&order.r#type, order.total.into()) {
                                    img {
                                        alt: o.clone(),
                                        src: "{ORDERS_ICONS.get(&order.r#type).unwrap()}",
                                    }
                                    br {}
                                }
                            }
                        }
                        td {
                            "{option.name}"
                            br {}
                            "{ generate_weapons_string(&metadata.weapons, &option.weapons) }"
                            EquipmentBox { equipment: option.equip.clone() }
                        }
                        td { "{option.swc}" }
                        td { "{option.points}" }
                    }
                }
            }
        }
    }
}

#[component]
fn EquipmentBox(equipment: Vec<Equipment>) -> Element {
    let metadata = consume_context::<Metadata>();
    rsx! {
        if !equipment.is_empty() {
            div {
                span { "Equipment:" }
                for e in equipment {
                    if let Some(label) = metadata
                        .equips
                        .iter()
                        .find_map(|eq| if eq.id == e.id { Some(eq.clone()) } else { None })
                    {
                        WikiLinkLabel { label }
                    }
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
        if !skills.is_empty() {
            div {
                span { "Skills:" }
                for e in skills {
                    if let Some(label) = metadata
                        .skills
                        .iter()
                        .find_map(|eq| if eq.id == e.id { Some(eq.clone()) } else { None })
                    {

                        WikiLinkLabel { label }
                    }
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
