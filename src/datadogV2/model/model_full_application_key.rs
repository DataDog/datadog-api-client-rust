// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Datadog application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FullApplicationKey {
    /// Attributes of a full application key.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::FullApplicationKeyAttributes>,
    /// ID of the application key.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Resources related to the application key.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::ApplicationKeyRelationships>,
    /// Application Keys resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ApplicationKeysType>,
}

impl FullApplicationKey {
    pub fn new() -> FullApplicationKey {
        FullApplicationKey {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::FullApplicationKeyAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::ApplicationKeyRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::ApplicationKeysType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for FullApplicationKey {
    fn default() -> Self {
        Self::new()
    }
}
