use std::{collections::HashMap, ops::Deref};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;
use wrapi::{http::Method, request::Request};

use super::{Id, Identifier, Relation};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventId(pub Id);

impl Deref for EventId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Lease,
    Booking,
}

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::Lease => "lease".into(),
            EventType::Booking => "booking".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Lease(Lease),
    Booking(Booking),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lease {
    pub id: Uuid,
    pub name: String,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub identifiers: Option<Vec<Identifier>>,
    pub leasee: Option<Vec<Relation>>,
    pub leasor: Option<Vec<Relation>>,
    pub lease_of: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Booking {
    pub id: Uuid,
    pub name: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub identifiers: Option<Vec<Identifier>>,
    pub booked_by: Option<Relation>,
    pub lease: Option<Relation>,
    pub room: Option<Relation>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NewEvent {
    #[serde(rename_all = "camelCase")]
    Lease {
        name: String,
        start: String,
        end: Option<String>,
        identifiers: Option<Vec<Identifier>>,
        leasee: Option<Vec<Id>>,
        leasor: Option<Vec<Id>>,
        lease_of: Option<Vec<Id>>,
    },
    #[serde(rename_all = "camelCase")]
    Booking {
        name: String,
        start: String,
        end: String,
        identifiers: Option<Vec<Identifier>>,
        booked_by: Option<Id>,
        lease: Id,
        room: Option<Id>,
    },
}

/// Get events of provided type
impl Request<Vec<Event>> for EventType {
    fn endpoint(&self) -> String {
        "event".into()
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

/// Create event
impl Request<Id> for NewEvent {
    fn endpoint(&self) -> String {
        "event".into()
    }

    fn method(&self) -> Method {
        Method::POST
    }
}

/// Get event by id
impl Request<Event> for Id {
    fn endpoint(&self) -> String {
        format!("event/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}

/// Delete event by id
impl Request<()> for EventId {
    fn endpoint(&self) -> String {
        format!("event/{}", self.deref().id)
    }

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}
