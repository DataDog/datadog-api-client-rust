// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident Team data for an update request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTeamUpdateData {
    /// The incident team's attributes for an update request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IncidentTeamUpdateAttributes>,
    /// The incident team's ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The incident team's relationships.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::IncidentTeamRelationships>,
    /// Incident Team resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentTeamType,
}

impl IncidentTeamUpdateData {
    pub fn new(type_: crate::datadogV2::model::IncidentTeamType) -> IncidentTeamUpdateData {
        IncidentTeamUpdateData {
            attributes: None,
            id: None,
            relationships: None,
            type_,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::IncidentTeamUpdateAttributes,
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
        value: crate::datadogV2::model::IncidentTeamRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }
}
