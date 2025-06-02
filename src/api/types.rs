use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub factions: Vec<Faction>,
    pub ammunitions: Vec<WikiItem>,
    pub weapons: Vec<Weapon>,
    pub skills: Vec<WikiItem>,
    pub equips: Vec<WikiItem>,
    // pub hack: Vec<Hack>,
    // pub martialArts: Vec<MartialArt>,
    // pub metachemistry: Vec<Metachemistry>,
    // pub booty: Vec<Booty>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    pub id: u64,
    pub r#type: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiItem {
    pub id: u64,
    pub name: String,
    pub wiki: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub id: u64,
    pub idArmy: u64,
    pub name: String,
    pub profileGroups: Vec<ProfileGroup>,
    pub filters: Filter,
    pub factions: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileGroup {
    pub isc: Option<String>,
    pub profiles: Vec<Profile>,
    pub options: Vec<UnitOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub bts: i8,
    pub cc: i8,
    pub r#move: Vec<i8>,
    pub r#type: u8,
    pub ava: i8,
    pub str: bool,
    pub bs: i8,
    pub s: i8,
    pub w: i8,
    pub ph: i8,
    pub arm: i8,
    pub wip: i8,
    pub equip: Vec<Equipment>,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub id: u64,
    pub order: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    pub id: u64,
    pub order: u8,
    pub extra: Option<Vec<u64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resume {
    pub id: u64,
    pub idArmy: u64,
    pub name: String,
    pub logo: String,
    pub r#type: u64,
    pub category: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnitOption {
    pub id: u64,
    pub points: u64,
    pub name: String,
    pub swc: String,
    pub weapons: Vec<WeaponRef>,
    pub equip: Vec<Equipment>,
    pub orders: Vec<Order>,
    // pub skills: Vec<WikiItem>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub r#type: String,
    pub total: u8,
}

impl Ord for UnitOption {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.points.cmp(&other.points)
    }
}

impl PartialOrd for UnitOption {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.points.partial_cmp(&other.points)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WeaponRef {
    pub id: Option<u64>,
    pub order: Option<i64>,
    extra: Option<Vec<i64>>,
}

impl Ord for WeaponRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for WeaponRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}
