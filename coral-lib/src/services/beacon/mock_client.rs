use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::http::StatusCode;

use crate::error::{AppServerResult, ServerErrorResponse};
use crate::services::beacon::types;

use super::client::BeaconClientTrait;

#[derive(Clone, Debug, Default)]
pub struct MockBeaconClient {
    pub block_responses: Arc<Mutex<VecDeque<types::BlockResponse>>>,
    pub block_root_responses: Arc<Mutex<VecDeque<String>>>,
    pub validator_responses: Arc<Mutex<VecDeque<types::ValidatorResponse>>>,
    pub validators_by_pubkey_responses: Arc<Mutex<VecDeque<types::ValidatorListResponse>>>,
}

impl MockBeaconClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_block_response(&mut self, value: types::BlockResponse) {
        self.block_responses.lock().unwrap().push_back(value);
    }

    pub fn push_block_root_response(&mut self, value: String) {
        self.block_root_responses.lock().unwrap().push_back(value);
    }

    pub fn push_validator_response(&mut self, value: types::ValidatorResponse) {
        self.validator_responses.lock().unwrap().push_back(value);
    }

    pub fn push_validators_by_pubkey_response(&mut self, value: types::ValidatorListResponse) {
        self.validators_by_pubkey_responses
            .lock()
            .unwrap()
            .push_back(value);
    }
}

#[async_trait]
impl BeaconClientTrait for MockBeaconClient {
    async fn fetch_block(
        &self,
        _state_id: types::BlockId,
    ) -> AppServerResult<types::BlockResponse> {
        match self.block_responses.lock().unwrap().pop_front() {
            Some(block) => Ok(block),
            None => Err(ServerErrorResponse::new(
                StatusCode::IM_A_TEAPOT,
                2000,
                "No fetch_block response set",
            )),
        }
    }

    async fn fetch_block_root(&self, _state_id: types::BlockId) -> AppServerResult<String> {
        match self.block_root_responses.lock().unwrap().pop_front() {
            Some(val) => Ok(val),
            None => Err(ServerErrorResponse::new(
                StatusCode::IM_A_TEAPOT,
                2000,
                "No fetch_block_root response set",
            )),
        }
    }

    async fn fetch_validator(
        &self,
        _state_id: types::StateId,
        _validator: types::ValidatorId,
    ) -> AppServerResult<types::ValidatorResponse> {
        match self.validator_responses.lock().unwrap().pop_front() {
            Some(value) => Ok(value),
            None => Err(ServerErrorResponse::new(
                StatusCode::IM_A_TEAPOT,
                2000,
                "No fetch_validator response set",
            )),
        }
    }

    async fn fetch_validators_by_pubkey(
        &self,
        _state_id: types::StateId,
        _validators: &[String],
    ) -> AppServerResult<types::ValidatorListResponse> {
        match self
            .validators_by_pubkey_responses
            .lock()
            .unwrap()
            .pop_front()
        {
            Some(value) => Ok(value),
            None => Err(ServerErrorResponse::new(
                StatusCode::IM_A_TEAPOT,
                2000,
                "No fetch_validators_by_pubkey response set",
            )),
        }
    }

    async fn fetch_validators_by_index(
        &self,
        _state_id: types::StateId,
        _validators: &[u64],
    ) -> AppServerResult<types::ValidatorListResponse> {
        Err(ServerErrorResponse::new(
            StatusCode::IM_A_TEAPOT,
            2000,
            "No fetch_validators_by_index response set",
        ))
    }
}
