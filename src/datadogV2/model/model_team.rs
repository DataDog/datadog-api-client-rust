// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A team
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Team {
    /// Team attributes
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::TeamAttributes>,
    /// The team's identifier
    #[serde(rename = "id")]
    pub id: String,
    /// Resources related to a team
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::TeamRelationships>>,
    /// Team type
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TeamType,
}

impl Team {
    pub fn new(
        attributes: Box<crate::datadogV2::model::TeamAttributes>,
        id: String,
        type_: crate::datadogV2::model::TeamType,
    ) -> Team {
        Team {
            attributes,
            id,
            relationships: None,
            type_,
        }
    }
}