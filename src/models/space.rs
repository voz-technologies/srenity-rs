use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;
use wrapi::{http::Method, request::Request};

use super::{Id, Identifier, Relation};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub atype: String,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub gross_area: Option<f64>,
    pub net_area: Option<f64>,
    pub rentable_area: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Capacity {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub atype: String,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub max_occupancy: Option<f64>,
    pub seating_capacity: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SpaceType {
    AccessControlZone,
    Building,
    Level,
    Room,
    Entrance,
}

impl ToString for SpaceType {
    fn to_string(&self) -> String {
        match self {
            SpaceType::AccessControlZone => "access_control_zone".to_string(),
            SpaceType::Building => "building".to_string(),
            SpaceType::Level => "level".to_string(),
            SpaceType::Room => "room".to_string(),
            SpaceType::Entrance => "entrance".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Space {
    AccessControlZone(AccessControlZone),
    Building(Building),
    Level(Level),
    Room(Room),
    Entrance(Entrance),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessControlZone {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub has_part: Option<Vec<Relation>>,
    pub is_part_of: Option<Vec<Relation>>,
    pub is_location_of: Option<Vec<Relation>>,
    pub area: Option<Area>,
    pub capacity: Option<Capacity>,
    pub address: Option<Vec<Relation>>,
    pub included_in: Option<Vec<Relation>>,
    pub has_point: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Building {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub has_part: Option<Vec<Relation>>,
    pub is_part_of: Option<Vec<Relation>>,
    pub is_location_of: Option<Vec<Relation>>,
    pub area: Option<Area>,
    pub capacity: Option<Capacity>,
    pub address: Option<Vec<Relation>>,
    pub included_in: Option<Vec<Relation>>,
    pub has_point: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub has_part: Option<Vec<Relation>>,
    pub is_part_of: Option<Vec<Relation>>,
    pub is_location_of: Option<Vec<Relation>>,
    pub area: Option<Area>,
    pub capacity: Option<Capacity>,
    pub address: Option<Vec<Relation>>,
    pub included_in: Option<Vec<Relation>>,
    pub has_point: Option<Vec<Relation>>,
    pub level_number: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub has_part: Option<Vec<Relation>>,
    pub is_part_of: Option<Vec<Relation>>,
    pub is_location_of: Option<Vec<Relation>>,
    pub area: Option<Area>,
    pub capacity: Option<Capacity>,
    pub address: Option<Vec<Relation>>,
    pub included_in: Option<Vec<Relation>>,
    pub has_point: Option<Vec<Relation>>,
    pub bookable: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entrance {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub has_part: Option<Vec<Relation>>,
    pub is_part_of: Option<Vec<Relation>>,
    pub is_location_of: Option<Vec<Relation>>,
    pub area: Option<Area>,
    pub capacity: Option<Capacity>,
    pub address: Option<Vec<Relation>>,
    pub included_in: Option<Vec<Relation>>,
    pub has_point: Option<Vec<Relation>>,
    pub bookable: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NewSpace {
    #[serde(rename_all = "camelCase")]
    AccessControlZone {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        has_part: Option<Vec<Relation>>,
        is_part_of: Option<Vec<Relation>>,
        is_location_of: Option<Vec<Relation>>,
        area: Option<Area>,
        capacity: Option<Capacity>,
        address: Option<Vec<Relation>>,
        included_in: Option<Vec<Relation>>,
        has_point: Option<Vec<Relation>>,
    },

    #[serde(rename_all = "camelCase")]
    Building {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        has_part: Option<Vec<Relation>>,
        is_part_of: Option<Vec<Relation>>,
        is_location_of: Option<Vec<Relation>>,
        area: Option<Area>,
        capacity: Option<Capacity>,
        address: Option<Vec<Relation>>,
        included_in: Option<Vec<Relation>>,
        has_point: Option<Vec<Relation>>,
    },

    #[serde(rename_all = "camelCase")]
    Level {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        has_part: Option<Vec<Relation>>,
        is_part_of: Option<Vec<Relation>>,
        is_location_of: Option<Vec<Relation>>,
        area: Option<Area>,
        capacity: Option<Capacity>,
        address: Option<Vec<Relation>>,
        included_in: Option<Vec<Relation>>,
        has_point: Option<Vec<Relation>>,
        level_number: Option<u32>,
    },

    #[serde(rename_all = "camelCase")]
    Room {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        has_part: Option<Vec<Relation>>,
        is_part_of: Option<Vec<Relation>>,
        is_location_of: Option<Vec<Relation>>,
        area: Option<Area>,
        capacity: Option<Capacity>,
        address: Option<Vec<Relation>>,
        included_in: Option<Vec<Relation>>,
        has_point: Option<Vec<Relation>>,
        bookable: Option<bool>,
    },

    #[serde(rename_all = "camelCase")]
    Entrance {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        has_part: Option<Vec<Relation>>,
        is_part_of: Option<Vec<Relation>>,
        is_location_of: Option<Vec<Relation>>,
        area: Option<Area>,
        capacity: Option<Capacity>,
        address: Option<Vec<Relation>>,
        included_in: Option<Vec<Relation>>,
        has_point: Option<Vec<Relation>>,
        bookable: Option<bool>,
    },
}

/// Get spaces of provided type
impl Request<Vec<Space>> for SpaceType {
    fn endpoint(&self) -> String {
        "space".into()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Option<HashMap<String, String>> {
        Some(HashMap::from([("type".to_string(), self.to_string())]))
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}

/// Create space
impl Request<Id> for NewSpace {
    fn endpoint(&self) -> String {
        "space".into()
    }

    fn method(&self) -> Method {
        Method::POST
    }
}

/// Get space by id
impl Request<Space> for Id {
    fn endpoint(&self) -> String {
        format!("space/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}

/// Replace space by id
impl Request<()> for Space {
    fn endpoint(&self) -> String {
        let id = match self {
            Space::AccessControlZone(space) => space.id,
            Space::Building(space) => space.id,
            Space::Level(space) => space.id,
            Space::Room(space) => space.id,
            Space::Entrance(space) => space.id,
        };

        format!("space/{}", id)
    }

    fn method(&self) -> Method {
        Method::PUT
    }
}
