use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wrapi::{http::Method, request::Request};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub access_token: String,
}

impl Request<Auth> for AuthReq {
    fn endpoint(&self) -> String {
        "realms/Core/protocol/openid-connect/token".into()
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn form(&self) -> Option<HashMap<String, String>> {
        Some(HashMap::from([(
            "grant_type".into(),
            "client_credentials".into(),
        )]))
    }

    fn basic_auth(&self) -> Option<(String, Option<String>)> {
        Some((self.username.clone(), Some(self.password.clone())))
    }

    fn body(&self) -> Option<&Self> {
        None
    }
}
