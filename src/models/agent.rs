use super::{Id, Identifier, Relation};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap, ops::Deref};
use uuid::Uuid;
use wrapi::{http::Method, request::Request};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentId(pub Id);

impl Deref for AgentId {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AgentType {
    AccessGroup,
    Company,
    Department,
    Person,
}

impl ToString for AgentType {
    fn to_string(&self) -> String {
        match self {
            AgentType::AccessGroup => "access_group".into(),
            AgentType::Company => "company".into(),
            AgentType::Department => "department".into(),
            AgentType::Person => "person".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Agent {
    AccessGroup(AccessGroup),
    Company(Company),
    Department(Department),
    Person(Person),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessGroup {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub member_of: Option<Vec<Relation>>,
    pub logo: Option<String>,
    pub has_member: Option<Vec<Relation>>,
    pub includes_door: Option<Vec<Relation>>,
    pub includes_zone: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub member_of: Option<Vec<Relation>>,
    pub logo: Option<String>,
    pub has_member: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub member_of: Option<Vec<Relation>>,
    pub logo: Option<String>,
    pub has_member: Option<Vec<Relation>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub identifiers: Option<Vec<Identifier>>,
    pub member_of: Option<Vec<Relation>>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
    pub gender: Option<String>,
    pub image: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NewAgent {
    #[serde(rename_all = "camelCase")]
    AccessGroup {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        member_of: Option<Vec<Relation>>,
        logo: Option<String>,
        has_member: Option<Vec<Relation>>,
        includes_door: Option<Vec<Relation>>,
        includes_zone: Option<Vec<Relation>>,
    },
    #[serde(rename_all = "camelCase")]
    Company {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        member_of: Option<Vec<Relation>>,
        logo: Option<String>,
        has_member: Option<Vec<Relation>>,
    },
    #[serde(rename_all = "camelCase")]
    Department {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        member_of: Option<Vec<Relation>>,
        logo: Option<String>,
        has_member: Option<Vec<Relation>>,
    },
    #[serde(rename_all = "camelCase")]
    Person {
        name: String,
        identifiers: Option<Vec<Identifier>>,
        member_of: Option<Vec<Relation>>,
        family_name: Option<String>,
        given_name: Option<String>,
        gender: Option<String>,
        image: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    #[serde(rename = "type")]
    pub key_type: String,
    pub provider: String,
    pub key: String,
}

/// Get agents of provided type
impl Request<Vec<Agent>> for AgentType {
    fn endpoint(&self) -> String {
        "agent".into()
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

/// Create agent
impl Request<Id> for NewAgent {
    fn endpoint(&self) -> String {
        "agent".into()
    }

    fn method(&self) -> Method {
        Method::POST
    }
}

/// Get agent by id
impl Request<Agent> for Id {
    fn endpoint(&self) -> String {
        format!("agent/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}

/// Replace agent by id
impl Request<()> for Agent {
    fn endpoint(&self) -> String {
        let id = match self {
            Agent::AccessGroup(agent) => agent.id,
            Agent::Company(agent) => agent.id,
            Agent::Department(agent) => agent.id,
            Agent::Person(agent) => agent.id,
        };

        format!("agent/{}", id)
    }

    fn method(&self) -> Method {
        Method::PUT
    }
}

/// Delete agent by id
impl Request<()> for AgentId {
    fn endpoint(&self) -> String {
        format!("agent/{}", self.deref().id)
    }

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}

/// Get keys for agent by id
impl Request<Vec<Key>> for Id {
    fn endpoint(&self) -> String {
        format!("person/{}/keys", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}
