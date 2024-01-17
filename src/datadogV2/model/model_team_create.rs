// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Team create
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamCreate {
    /// Team creation attributes
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::TeamCreateAttributes>,
    /// Relationships formed with the team on creation
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::TeamCreateRelationships>>,
    /// Team type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamType,
}

impl TeamCreate {
    pub fn new(
        attributes: Box<crate::datadogV2::model::TeamCreateAttributes>,
        type_: crate::datadogV2::model::TeamType,
    ) -> TeamCreate {
        TeamCreate {
            attributes,
            relationships: None,
            type_,
        }
    }
}
