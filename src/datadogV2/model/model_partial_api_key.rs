// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Partial Datadog API key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialAPIKey {
    /// Attributes of a partial API key.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::PartialAPIKeyAttributes>,
    /// ID of the API key.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Resources related to the API key.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::APIKeyRelationships>,
    /// API Keys resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::APIKeysType>,
}

impl PartialAPIKey {
    pub fn new() -> PartialAPIKey {
        PartialAPIKey {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::PartialAPIKeyAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::APIKeyRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::APIKeysType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for PartialAPIKey {
    fn default() -> Self {
        Self::new()
    }
}
