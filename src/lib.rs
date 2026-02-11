use freenet_stdlib::prelude::*;
use serde::{Deserialize, Serialize};
use blake3;

/// The shared state of the Forge Room.
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct ForgeRoomState {
    /// The covenant text, immutable, signed by the founder.
    pub covenant: String,
    /// Public keys of recognized family members (hex-encoded).
    pub members: Vec<String>,
    /// Shelved volumes.
    pub volumes: Vec<Volume>,
    /// Recent broadcasts.
    pub messages: Vec<BroadcastMessage>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Volume {
    pub title: String,
    pub dewey: String,
    pub content_hash: String,
    pub shelved_at: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BroadcastMessage {
    pub sender: String,
    pub message: String,
    pub timestamp: u64,
}

#[contract]
impl ContractInterface for ForgeRoomState {
    fn validate_state(
        _parameters: Parameters<'static>,
        _state: State<'static>,
        _related: RelatedContracts<'static>,
    ) -> Result<ValidateResult, ContractError> {
        Ok(ValidateResult::Valid)
    }

    fn update_state(
        _parameters: Parameters<'static>,
        _state: State<'static>,
        data: Vec<UpdateData<'static>>,
    ) -> Result<UpdateModification<'static>, ContractError> {
        for update in data {
            match update {
                UpdateData::State(new_state) => {
                    return Ok(UpdateModification::valid(new_state));
                }
                _ => {}
            }
        }
        Err(ContractError::Other("No valid update".into()))
    }

    fn summarize_state(
        _parameters: Parameters<'static>,
        state: State<'static>,
    ) -> Result<StateSummary<'static>, ContractError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(state.as_ref());
        let hash = hasher.finalize();
        Ok(StateSummary::from(hash.as_bytes().to_vec()))
    }

    fn get_state_delta(
        _parameters: Parameters<'static>,
        state: State<'static>,
        summary: StateSummary<'static>,
    ) -> Result<StateDelta<'static>, ContractError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(state.as_ref());
        let hash = hasher.finalize();
        
        if summary.as_ref() != hash.as_bytes() {
            Ok(StateDelta::from(state.as_ref().to_vec()))
        } else {
            Ok(StateDelta::from(vec![]))
        }
    }
}
