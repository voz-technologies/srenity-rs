use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;
use wrapi::{http::Method, request::Request};

use super::{Id, Identifier, Relation};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CollectionType {
    Apartment,
    Premises,
    RealEstate,
}

impl ToString for CollectionType {
    fn to_string(&self) -> String {
        match self {
            CollectionType::Apartment => "apartment".into(),
            CollectionType::Premises => "premises".into(),
            CollectionType::RealEstate => "real_estate".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Collection {
    Apartment(Apartment),
    Premises(Premises),
    RealEstate(RealEstate),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Apartment {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub includes: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Premises {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub includes: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RealEstate {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub includes: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NewCollection {
    #[serde(rename_all = "camelCase")]
    Apartment {
        name: String,
        includes: Option<Vec<Relation>>,
    },
    #[serde(rename_all = "camelCase")]
    Premises {
        name: String,
        includes: Option<Vec<Relation>>,
    },
    #[serde(rename_all = "camelCase")]
    RealEstate {
        name: String,
        includes: Option<Vec<Relation>>,
    },
}

/// Get collections of provided type
impl Request<Vec<Collection>> for CollectionType {
    fn endpoint(&self) -> String {
        "collection".into()
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

/// Create collection
impl Request<Id> for NewCollection {
    fn endpoint(&self) -> String {
        "collection".into()
    }

    fn method(&self) -> Method {
        Method::POST
    }
}

/// Get collection by id
impl Request<Collection> for Id {
    fn endpoint(&self) -> String {
        format!("collection/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}

/// Replace collection by id
impl Request<()> for Collection {
    fn endpoint(&self) -> String {
        let id = match self {
            Collection::Apartment(collection) => collection.id,
            Collection::Premises(collection) => collection.id,
            Collection::RealEstate(collection) => collection.id,
        };

        format!("collection/{}", id)
    }

    fn method(&self) -> Method {
        Method::PUT
    }
}
