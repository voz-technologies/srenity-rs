use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;
use wrapi::{http::Method, request::Request};

use super::{Id, Identifier};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InformationType {
    ArchitectureArea,
    ArchitectureCapacity,
    PostalAddress,
}

impl ToString for InformationType {
    fn to_string(&self) -> String {
        match self {
            InformationType::ArchitectureArea => "architecture_area".into(),
            InformationType::ArchitectureCapacity => "architecture_capacity".into(),
            InformationType::PostalAddress => "postal_address".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Information {
    ArchitectureArea(ArchitectureArea),
    ArchitectureCapacity(ArchitectureCapacity),
    PostalAddress(PostalAddress),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureArea {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub gross_area: Option<f32>,
    pub net_area: Option<f32>,
    pub rentable_area: Option<f32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureCapacity {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub max_occupancy: Option<u32>,
    pub seating_capacity: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostalAddress {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub region: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NewInformation {
    #[serde(rename_all = "camelCase")]
    ArchitectureArea {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        gross_area: Option<f32>,
        net_area: Option<f32>,
        rentable_area: Option<f32>,
    },
    #[serde(rename_all = "camelCase")]
    ArchitectureCapacity {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        max_occupancy: Option<u32>,
        seating_capacity: Option<u32>,
    },
    #[serde(rename_all = "camelCase")]
    PostalAddress {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        address_line1: Option<String>,
        address_line2: Option<String>,
        city: Option<String>,
        country: Option<String>,
        postal_code: Option<String>,
        region: Option<String>,
    },
}

/// Get information of provided type
impl Request<Vec<Information>> for InformationType {
    fn endpoint(&self) -> String {
        "information".into()
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

/// Create information
impl Request<Id> for NewInformation {
    fn endpoint(&self) -> String {
        "information".into()
    }

    fn method(&self) -> Method {
        Method::POST
    }
}

/// Get information by id
impl Request<Information> for Id {
    fn endpoint(&self) -> String {
        format!("information/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}

/// Replace information by id
impl Request<()> for Information {
    fn endpoint(&self) -> String {
        let id = match self {
            Information::ArchitectureArea(information) => information.id,
            Information::ArchitectureCapacity(information) => information.id,
            Information::PostalAddress(information) => information.id,
        };

        format!("information/{}", id)
    }

    fn method(&self) -> Method {
        Method::PUT
    }
}
