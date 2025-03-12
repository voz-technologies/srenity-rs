use std::str::{self, FromStr};

use agent::{Agent, AgentType};
use asset::{Asset, AssetType};
use event::{Event, EventType};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use space::{Space, SpaceType};
use uuid::Uuid;

use crate::error::Error;

pub mod agent;
pub mod asset;
pub mod auth;
pub mod collection;
pub mod event;
pub mod information;
pub mod space;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id {
    pub id: Uuid,
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self { id: value }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub integration: String,
    pub external_id: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub rtype: Option<String>,
    pub name: Option<String>,
}

impl From<Uuid> for Relation {
    fn from(value: Uuid) -> Self {
        Self {
            id: value,
            rtype: None,
            name: None,
        }
    }
}

impl TryFrom<String> for Relation {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&value)
                .map_err(|_| Error::Unknown("Failed to parse UUID".into()))?,
            rtype: None,
            name: None,
        })
    }
}

impl From<Agent> for Relation {
    fn from(value: Agent) -> Self {
        match value {
            Agent::AccessGroup(agent) => Self {
                id: agent.id,
                rtype: Some(AgentType::AccessGroup.to_string()),
                name: Some(agent.name),
            },
            Agent::Company(agent) => Self {
                id: agent.id,
                rtype: Some(AgentType::Company.to_string()),
                name: Some(agent.name),
            },
            Agent::Department(agent) => Self {
                id: agent.id,
                rtype: Some(AgentType::Department.to_string()),
                name: Some(agent.name),
            },
            Agent::Person(agent) => Self {
                id: agent.id,
                rtype: Some(AgentType::Person.to_string()),
                name: Some(agent.name),
            },
        }
    }
}

impl From<Asset> for Relation {
    fn from(value: Asset) -> Self {
        match value {
            Asset::Door(asset) => Self {
                id: asset.id,
                rtype: Some(AssetType::Door.to_string()),
                name: Some(asset.name),
            },
            Asset::Meter(asset) => Self {
                id: asset.id,
                rtype: Some(AssetType::Meter.to_string()),
                name: Some(asset.name),
            },
            Asset::ElectricalMeter(asset) => Self {
                id: asset.id,
                rtype: Some(AssetType::ElectricalMeter.to_string()),
                name: Some(asset.name),
            },
            Asset::GasMeter(asset) => Self {
                id: asset.id,
                rtype: Some(AssetType::GasMeter.to_string()),
                name: Some(asset.name),
            },
            Asset::HotWaterMeter(asset) => Self {
                id: asset.id,
                rtype: Some(AssetType::HotWaterMeter.to_string()),
                name: Some(asset.name),
            },
            Asset::ChilledWaterMeter(asset) => Self {
                id: asset.id,
                rtype: Some(AssetType::ChilledWaterMeter.to_string()),
                name: Some(asset.name),
            },
        }
    }
}

impl From<Event> for Relation {
    fn from(value: Event) -> Self {
        match value {
            Event::Lease(event) => Self {
                id: event.id,
                rtype: Some(EventType::Lease.to_string()),
                name: Some(event.name),
            },
            Event::Booking(event) => Self {
                id: event.id,
                rtype: Some(EventType::Booking.to_string()),
                name: Some(event.name),
            },
        }
    }
}

impl From<Space> for Relation {
    fn from(value: Space) -> Self {
        match value {
            Space::AccessControlZone(space) => Self {
                id: space.id,
                rtype: Some(SpaceType::AccessControlZone.to_string()),
                name: Some(space.name),
            },
            Space::Building(space) => Self {
                id: space.id,
                rtype: Some(SpaceType::Building.to_string()),
                name: Some(space.name),
            },
            Space::Level(space) => Self {
                id: space.id,
                rtype: Some(SpaceType::Level.to_string()),
                name: Some(space.name),
            },
            Space::Room(space) => Self {
                id: space.id,
                rtype: Some(SpaceType::Room.to_string()),
                name: Some(space.name),
            },
            Space::Entrance(space) => Self {
                id: space.id,
                rtype: Some(SpaceType::Entrance.to_string()),
                name: Some(space.name),
            },
        }
    }
}
