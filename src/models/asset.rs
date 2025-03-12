use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;
use wrapi::{http::Method, request::Request};

use super::{Id, Identifier, Relation};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetType {
    Door,
    Meter,
    ElectricalMeter,
    GasMeter,
    HotWaterMeter,
    ChilledWaterMeter,
}

impl ToString for AssetType {
    fn to_string(&self) -> String {
        match self {
            AssetType::Door => "door".into(),
            AssetType::Meter => "meter".into(),
            AssetType::ElectricalMeter => "electrical_meter".into(),
            AssetType::GasMeter => "gas_meter".into(),
            AssetType::HotWaterMeter => "hot_water_meter".into(),
            AssetType::ChilledWaterMeter => "chilled_water_meter".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Asset {
    Door(Door),
    Meter(Meter),
    ElectricalMeter(ElectricalMeter),
    GasMeter(GasMeter),
    HotWaterMeter(HotWaterMeter),
    ChilledWaterMeter(ChilledWaterMeter),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Door {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub initial_cost: Option<String>,
    pub installation_date: Option<String>, // TODO: DateTime
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub maintenance_interval: Option<u32>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub turnover_date: Option<String>, // TODO: DateTime
    pub weight: Option<String>,
    pub located_in: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meter {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub initial_cost: Option<String>,
    pub installation_date: Option<String>, // TODO: DateTime
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub maintenance_interval: Option<u32>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub turnover_date: Option<String>, // TODO: DateTime
    pub weight: Option<String>,
    pub located_in: Option<Vec<Relation>>,
    pub operational_stage_count: Option<String>,
    pub feeds: Option<Vec<Relation>>,
    pub is_virtual_meter: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalMeter {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub initial_cost: Option<String>,
    pub installation_date: Option<String>, // TODO: DateTime
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub maintenance_interval: Option<u32>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub turnover_date: Option<String>, // TODO: DateTime
    pub weight: Option<String>,
    pub located_in: Option<Vec<Relation>>,
    pub operational_stage_count: Option<String>,
    pub feeds: Option<Vec<Relation>>,
    pub is_virtual_meter: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GasMeter {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub initial_cost: Option<String>,
    pub installation_date: Option<String>, // TODO: DateTime
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub maintenance_interval: Option<u32>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub turnover_date: Option<String>, // TODO: DateTime
    pub weight: Option<String>,
    pub located_in: Option<Vec<Relation>>,
    pub operational_stage_count: Option<String>,
    pub feeds: Option<Vec<Relation>>,
    pub is_virtual_meter: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HotWaterMeter {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub initial_cost: Option<String>,
    pub installation_date: Option<String>, // TODO: DateTime
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub maintenance_interval: Option<u32>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub turnover_date: Option<String>, // TODO: DateTime
    pub weight: Option<String>,
    pub located_in: Option<Vec<Relation>>,
    pub operational_stage_count: Option<String>,
    pub feeds: Option<Vec<Relation>>,
    pub is_virtual_meter: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChilledWaterMeter {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub initial_cost: Option<String>,
    pub installation_date: Option<String>, // TODO: DateTime
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub maintenance_interval: Option<u32>,
    pub model_number: Option<String>,
    pub serial_number: Option<String>,
    pub turnover_date: Option<String>, // TODO: DateTime
    pub weight: Option<String>,
    pub located_in: Option<Vec<Relation>>,
    pub operational_stage_count: Option<String>,
    pub feeds: Option<Vec<Relation>>,
    pub is_virtual_meter: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NewAsset {
    #[serde(rename_all = "camelCase")]
    Door {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        initial_cost: Option<String>,
        installation_date: Option<String>, // TODO: DateTime
        ip_address: Option<String>,
        mac_address: Option<String>,
        maintenance_interval: Option<u32>,
        model_number: Option<String>,
        serial_number: Option<String>,
        turnover_date: Option<String>, // TODO: DateTime
        weight: Option<String>,
        located_in: Option<Vec<Relation>>,
    },
    #[serde(rename_all = "camelCase")]
    Meter {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        initial_cost: Option<String>,
        installation_date: Option<String>, // TODO: DateTime
        ip_address: Option<String>,
        mac_address: Option<String>,
        maintenance_interval: Option<u32>,
        model_number: Option<String>,
        serial_number: Option<String>,
        turnover_date: Option<String>, // TODO: DateTime
        weight: Option<String>,
        located_in: Option<Vec<Relation>>,
        operational_stage_count: Option<String>,
        feeds: Option<Vec<Relation>>,
        is_virtual_meter: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    ElectricalMeter {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        initial_cost: Option<String>,
        installation_date: Option<String>, // TODO: DateTime
        ip_address: Option<String>,
        mac_address: Option<String>,
        maintenance_interval: Option<u32>,
        model_number: Option<String>,
        serial_number: Option<String>,
        turnover_date: Option<String>, // TODO: DateTime
        weight: Option<String>,
        located_in: Option<Vec<Relation>>,
        operational_stage_count: Option<String>,
        feeds: Option<Vec<Relation>>,
        is_virtual_meter: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    GasMeter {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        initial_cost: Option<String>,
        installation_date: Option<String>, // TODO: DateTime
        ip_address: Option<String>,
        mac_address: Option<String>,
        maintenance_interval: Option<u32>,
        model_number: Option<String>,
        serial_number: Option<String>,
        turnover_date: Option<String>, // TODO: DateTime
        weight: Option<String>,
        located_in: Option<Vec<Relation>>,
        operational_stage_count: Option<String>,
        feeds: Option<Vec<Relation>>,
        is_virtual_meter: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    HotWaterMeter {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        initial_cost: Option<String>,
        installation_date: Option<String>, // TODO: DateTime
        ip_address: Option<String>,
        mac_address: Option<String>,
        maintenance_interval: Option<u32>,
        model_number: Option<String>,
        serial_number: Option<String>,
        turnover_date: Option<String>, // TODO: DateTime
        weight: Option<String>,
        located_in: Option<Vec<Relation>>,
        operational_stage_count: Option<String>,
        feeds: Option<Vec<Relation>>,
        is_virtual_meter: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    ChilledWaterMeter {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        initial_cost: Option<String>,
        installation_date: Option<String>, // TODO: DateTime
        ip_address: Option<String>,
        mac_address: Option<String>,
        maintenance_interval: Option<u32>,
        model_number: Option<String>,
        serial_number: Option<String>,
        turnover_date: Option<String>, // TODO: DateTime
        weight: Option<String>,
        located_in: Option<Vec<Relation>>,
        operational_stage_count: Option<String>,
        feeds: Option<Vec<Relation>>,
        is_virtual_meter: Option<bool>,
    },
}

/// Get assets of provided type
impl Request<Vec<Asset>> for AssetType {
    fn endpoint(&self) -> String {
        "asset".into()
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

/// Create asset
impl Request<Id> for NewAsset {
    fn endpoint(&self) -> String {
        "asset".into()
    }

    fn method(&self) -> Method {
        Method::POST
    }
}

/// Get asset by id
impl Request<Asset> for Id {
    fn endpoint(&self) -> String {
        format!("asset/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}

/// Replace asset by id
impl Request<()> for Asset {
    fn endpoint(&self) -> String {
        let id = match self {
            Asset::Door(asset) => asset.id,
            Asset::Meter(asset) => asset.id,
            Asset::ElectricalMeter(asset) => asset.id,
            Asset::GasMeter(asset) => asset.id,
            Asset::HotWaterMeter(asset) => asset.id,
            Asset::ChilledWaterMeter(asset) => asset.id,
        };

        format!("asset/{}", id)
    }

    fn method(&self) -> Method {
        Method::PUT
    }
}
