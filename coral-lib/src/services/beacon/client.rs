use async_trait::async_trait;

use crate::error::AppServerResult;
use crate::services::beacon;
use crate::services::beacon::types;

use super::types::{BlockId, StateId, ValidatorId};

#[async_trait]
pub trait BeaconClientTrait {
    async fn fetch_block(&self, block_id: BlockId) -> AppServerResult<types::BlockResponse>;

    async fn fetch_block_root(&self, block_id: BlockId) -> AppServerResult<String>;

    async fn fetch_validator(
        &self,
        state_id: types::StateId,
        validator: ValidatorId,
    ) -> AppServerResult<types::ValidatorResponse>;

    async fn fetch_validators_by_pubkey(
        &self,
        state_id: StateId,
        validators: &[String],
    ) -> AppServerResult<types::ValidatorListResponse>;

    async fn fetch_validators_by_index(
        &self,
        state_id: StateId,
        validators: &[u64],
    ) -> AppServerResult<types::ValidatorListResponse>;
}

#[derive(Clone, Debug)]
pub struct BeaconClient {
    pub url: String,
}

impl BeaconClient {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[async_trait]
impl BeaconClientTrait for BeaconClient {
    async fn fetch_block(&self, block_id: BlockId) -> AppServerResult<types::BlockResponse> {
        beacon::fetch_block::fetch_block(&self.url, block_id).await
    }

    async fn fetch_block_root(&self, block_id: BlockId) -> AppServerResult<String> {
        beacon::fetch_block::fetch_block_root(&self.url, block_id).await
    }

    async fn fetch_validator(
        &self,
        state_id: types::StateId,
        validator: ValidatorId,
    ) -> AppServerResult<types::ValidatorResponse> {
        beacon::fetch_validator::fetch_validator(&self.url, state_id, validator).await
    }

    async fn fetch_validators_by_pubkey(
        &self,
        state_id: StateId,
        validators: &[String],
    ) -> AppServerResult<types::ValidatorListResponse> {
        beacon::fetch_validators::fetch_validators_by_pubkey(&self.url, state_id, validators).await
    }

    async fn fetch_validators_by_index(
        &self,
        state_id: StateId,
        validators: &[u64],
    ) -> AppServerResult<types::ValidatorListResponse> {
        beacon::fetch_validators::fetch_validators_by_index(&self.url, state_id, validators).await
    }
}
