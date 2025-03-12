use std::fmt;

use serde::de::DeserializeOwned;
use uuid::Uuid;
use wrapi::{request::Request, reqwest::Client};

use crate::{
    error::Error,
    models::{
        agent::{Agent, AgentId, AgentType, Key, NewAgent},
        asset::{Asset, AssetType, NewAsset},
        auth::{Auth, AuthReq},
        collection::{Collection, CollectionType, NewCollection},
        event::{Event, EventId, EventType, NewEvent},
        information::{Information, InformationType, NewInformation},
        space::{NewSpace, Space, SpaceType},
        Id,
    },
};

#[derive(Clone, Debug)]
pub struct Handler {
    api_url: String,
    auth_url: String,
}

impl Handler {
    pub fn new(api_url: String, auth_url: String) -> Self {
        tracing::info!("Srenity auth url: {}", auth_url);
        tracing::info!("Srenity api url: {}", api_url);
        Self { api_url, auth_url }
    }

    /// Authenticate and if successful, saves the token
    /// in the handler for further use
    pub async fn auth(&self, client: &Client, payload: AuthReq) -> Result<Auth, Error> {
        tracing::debug!("{}: {:#?}", payload.method(), payload.endpoint(),);

        payload
            .send(client, &self.auth_url)
            .await
            .map_err(Error::from)
    }

    /// Get all agents of provided type
    pub async fn agents(
        &self,
        client: &Client,
        token: &str,
        agent_type: AgentType,
    ) -> Result<Vec<Agent>, Error> {
        self.send(client, agent_type, token).await
    }

    /// Create a new agent
    pub async fn create_agent(
        &self,
        client: &Client,
        token: &str,
        payload: NewAgent,
    ) -> Result<Uuid, Error> {
        let res = self.send(client, payload, token).await?;

        Ok(res.id)
    }

    /// Get agent with provided ID
    pub async fn agent(&self, client: &Client, token: &str, id: Uuid) -> Result<Agent, Error> {
        self.send(client, Id { id }, token).await
    }

    /// Replace an agent
    pub async fn replace_agent(
        &self,
        client: &Client,
        token: &str,
        payload: Agent,
    ) -> Result<(), Error> {
        let _ = self.send_opt(client, payload, token).await?;

        Ok(())
    }

    /// Delete an agent
    pub async fn delete_agent(&self, client: &Client, id: Uuid, token: &str) -> Result<(), Error> {
        let _ = self.send_opt(client, AgentId(Id { id }), token).await?;

        Ok(())
    }

    /// Get keys for person
    pub async fn person_keys(
        &self,
        client: &Client,
        token: &str,
        id: &Uuid,
    ) -> Result<Vec<Key>, Error> {
        self.send(client, Id { id: *id }, token).await
    }

    /// Get events of provided type
    pub async fn events(
        &self,
        client: &Client,
        token: &str,
        event_type: EventType,
    ) -> Result<Vec<Event>, Error> {
        self.send(client, event_type, token).await
    }

    /// Create an event
    pub async fn create_event(
        &self,
        client: &Client,
        token: &str,
        payload: NewEvent,
    ) -> Result<Uuid, Error> {
        let res = self.send(client, payload, token).await?;

        Ok(res.id)
    }

    /// Get event by id
    pub async fn event(&self, client: &Client, token: &str, id: &Uuid) -> Result<Event, Error> {
        self.send(client, Id { id: *id }, token).await
    }

    pub async fn delete_event(&self, client: &Client, token: &str, id: &Uuid) -> Result<(), Error> {
        let _ = self
            .send_opt(client, EventId(Id { id: *id }), token)
            .await?;

        Ok(())
    }

    /// Get spaces of provided type
    pub async fn spaces(
        &self,
        client: &Client,
        token: &str,
        space_type: SpaceType,
    ) -> Result<Vec<Space>, Error> {
        self.send(client, space_type, &token).await
    }

    /// Create a space
    pub async fn create_space(
        &self,
        client: &Client,
        token: &str,
        payload: NewSpace,
    ) -> Result<Uuid, Error> {
        let res = self.send(client, payload, token).await?;

        Ok(res.id)
    }

    /// Get space by id
    pub async fn space(&self, client: &Client, token: &str, id: &Uuid) -> Result<Space, Error> {
        self.send(client, Id { id: *id }, token).await
    }

    /// Replace a space
    pub async fn replace_space(
        &self,
        client: &Client,
        token: &str,
        payload: Space,
    ) -> Result<(), Error> {
        let _ = self.send_opt(client, payload, token).await?;

        Ok(())
    }

    /// Get assets of provided type
    pub async fn assets(
        &self,
        client: &Client,
        token: &str,
        asset_type: AssetType,
    ) -> Result<Vec<Asset>, Error> {
        self.send(client, asset_type, token).await
    }

    /// Create an asset
    pub async fn create_asset(
        &self,
        client: &Client,
        token: &str,
        payload: NewAsset,
    ) -> Result<Uuid, Error> {
        let res = self.send(client, payload, token).await?;

        Ok(res.id)
    }

    /// Get asset by id
    pub async fn asset(&self, client: &Client, token: &str, id: &Uuid) -> Result<Asset, Error> {
        self.send(client, Id { id: *id }, token).await
    }

    /// Replace an asset
    pub async fn replace_asset(
        &self,
        client: &Client,
        token: &str,
        payload: Asset,
    ) -> Result<(), Error> {
        let _ = self.send_opt(client, payload, token).await?;

        Ok(())
    }

    /// Get collections of provided type
    pub async fn collections(
        &self,
        client: &Client,
        token: &str,
        collection_type: CollectionType,
    ) -> Result<Vec<Collection>, Error> {
        self.send(client, collection_type, token).await
    }

    /// Create collection
    pub async fn create_collection(
        &self,
        client: &Client,
        token: &str,
        payload: NewCollection,
    ) -> Result<Uuid, Error> {
        let res = self.send(client, payload, token).await?;

        Ok(res.id)
    }

    /// Get collection by id
    pub async fn collection(
        &self,
        client: &Client,
        token: &str,
        id: &Uuid,
    ) -> Result<Collection, Error> {
        self.send(client, Id { id: *id }, token).await
    }

    /// Replace collection by id
    pub async fn replace_collection(
        &self,
        client: &Client,
        token: &str,
        payload: Collection,
    ) -> Result<(), Error> {
        let _ = self.send_opt(client, payload, token).await?;

        Ok(())
    }

    /// Get information of provided type
    pub async fn all_information(
        &self,
        client: &Client,
        token: &str,
        payload: InformationType,
    ) -> Result<Vec<Information>, Error> {
        self.send(client, payload, token).await
    }

    /// Create information
    pub async fn create_information(
        &self,
        client: &Client,
        token: &str,
        payload: NewInformation,
    ) -> Result<Uuid, Error> {
        let res = self.send(client, payload, token).await?;

        Ok(res.id)
    }

    /// Get information by id
    pub async fn information(
        &self,
        client: &Client,
        token: &str,
        id: &Uuid,
    ) -> Result<Information, Error> {
        self.send(client, Id { id: *id }, token).await
    }

    /// Update information
    pub async fn replace_information(
        &self,
        client: &Client,
        token: &str,
        payload: Information,
    ) -> Result<(), Error> {
        let _ = self.send_opt(client, payload, token).await?;

        Ok(())
    }

    // TODO: Check if we have a valid token
    // and if not, try to get one
    //
    /// Send a request to the Srenity API and
    /// deserialize the response into `U`
    async fn send<T, U>(&self, client: &Client, payload: T, token: &str) -> Result<U, Error>
    where
        T: Request<U> + fmt::Debug,
        U: DeserializeOwned + fmt::Debug + Send + Sync,
    {
        tracing::debug!(
            "{}: {:#?} with body {:#?}",
            payload.method(),
            payload.endpoint(),
            payload.body()
        );

        let x = payload
            .exec(payload.build(client, &self.api_url).bearer_auth(token))
            .await?;

        tracing::debug!(
            "Response for {}: {:#?}: {:#?}",
            payload.method(),
            payload.endpoint(),
            x
        );

        Ok(x)
    }

    /// Send a request to the Srenity API and
    /// deserialize the response into `Option<U>`
    async fn send_opt<T, U>(
        &self,
        client: &Client,
        payload: T,
        token: &str,
    ) -> Result<Option<U>, Error>
    where
        T: Request<U> + fmt::Debug,
        U: DeserializeOwned + fmt::Debug + Send + Sync,
    {
        tracing::debug!(
            "{}: {:#?} with body {:#?}",
            payload.method(),
            payload.endpoint(),
            payload.body()
        );

        let x = payload
            .exec_opt(payload.build(client, &self.api_url).bearer_auth(token))
            .await?;

        tracing::debug!(
            "Response for {}: {:#?}: {:#?}",
            payload.method(),
            payload.endpoint(),
            x
        );

        Ok(x)
    }
}
