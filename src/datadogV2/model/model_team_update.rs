// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team update request
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamUpdate {
    /// Team update attributes
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::TeamUpdateAttributes>,
    /// Team update relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::TeamUpdateRelationships>>,
    /// Team type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamType,
}

impl TeamUpdate {
    pub fn new(
        attributes: Box<crate::datadogV2::model::TeamUpdateAttributes>,
        type_: crate::datadogV2::model::TeamType,
    ) -> TeamUpdate {
        TeamUpdate {
            attributes,
            relationships: None,
            type_,
        }
    }
}
