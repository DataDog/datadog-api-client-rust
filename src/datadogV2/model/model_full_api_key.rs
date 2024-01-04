// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Datadog API key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullAPIKey {
    /// Attributes of a full API key.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::FullAPIKeyAttributes>>,
    /// ID of the API key.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Resources related to the API key.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::APIKeyRelationships>>,
    /// API Keys resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::APIKeysType>,
}

impl FullAPIKey {
    pub fn new() -> FullAPIKey {
        FullAPIKey {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}
