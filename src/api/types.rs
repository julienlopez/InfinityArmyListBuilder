use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub factions: Vec<Faction>,
    // pub ammunitions: Vec<Ammunition>,
    // pub weapons: Vec<Weapon>,
    // pub skills: Vec<Skill>,
    // pub equips: Vec<Equip>,
    // pub hack: Vec<Hack>,
    // pub martialArts: Vec<MartialArt>,
    // pub metachemistry: Vec<Metachemistry>,
    // pub booty: Vec<Booty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub id: u64,
    pub parent: u64,
    pub name: String,
    pub slug: String,
    pub discontinued: bool,
    pub logo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionData {
    pub version: String,
    pub units: Vec<Unit>,
    pub resume: Vec<Resume>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: u64,
    pub idArmy: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub id: u64,
    pub idArmy: u64,
    pub name: String,
    pub logo: String,
    pub r#type: u64,
    pub category: u64,
}
