// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Partial Datadog application key.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialApplicationKey {
    /// Attributes of a partial application key.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::PartialApplicationKeyAttributes>>,
    /// ID of the application key.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Resources related to the application key.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::ApplicationKeyRelationships>>,
    /// Application Keys resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ApplicationKeysType>,
}

impl PartialApplicationKey {
    pub fn new() -> PartialApplicationKey {
        PartialApplicationKey {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}