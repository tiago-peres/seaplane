//! Errors produced or propagated through the Seaplane SDK

use thiserror::Error;

#[cfg(feature = "api_v1")]
use crate::api::v1::{config::ConfigError, formations::FormationsError};

pub type Result<T> = std::result::Result<T, SeaplaneError>;

#[derive(Error, Debug)]
pub enum SeaplaneError {
    #[error("http error")]
    UnknownHttp(#[from] reqwest::Error),
    #[error("request did not include a required API key")]
    MissingRequestApiKey,
    #[error("request did not include a required authorization token")]
    MissingRequestAuthToken,
    #[error("request did not include the required formation name")]
    MissingFormationName,
    #[error("invalid URL")]
    UrlParse(#[from] url::ParseError),
    #[error("invalid json")]
    Json(#[from] serde_json::error::Error),
    #[error("request did not include any active configurations while force=false")]
    MissingActiveConfiguration,
    #[error("missing a required UUID")]
    MissingUuid,
    #[error("the given request conflict with one another")]
    ConflictingParams,
    #[error("flights cannot be empty")]
    EmptyFlights,
    #[error("missing required Flight name")]
    MissingFlightName,
    #[error("missing required Flight image reference")]
    MissingFlightImageReference,
    #[error("the requirements specified in the builder are in conflict and invalid")]
    ConflictingRequirements,
    #[error("request did not include the required key")]
    MissingConfigKey,
    #[error("request must target either key or range")]
    IncorrectConfigRequestTarget,
    #[cfg(feature = "api_v1")]
    #[error("the Formations Compute API returned an error status")]
    FormationsResponse(#[from] FormationsError),
    #[error("the Config Consensus API returned an error status")]
    #[cfg(feature = "api_v1")]
    ConfigResponse(#[from] ConfigError),
}
