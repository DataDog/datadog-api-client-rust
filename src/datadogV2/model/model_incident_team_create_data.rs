// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident Team data for a create request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTeamCreateData {
    /// The incident team's attributes for a create request.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::IncidentTeamCreateAttributes>>,
    /// The incident team's relationships.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::IncidentTeamRelationships>>,
    /// Incident Team resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentTeamType,
}

impl IncidentTeamCreateData {
    pub fn new(type_: crate::datadogV2::model::IncidentTeamType) -> IncidentTeamCreateData {
        IncidentTeamCreateData {
            attributes: None,
            relationships: None,
            type_,
        }
    }
}
