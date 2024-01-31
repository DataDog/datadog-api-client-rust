// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident Team data from a response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTeamResponseData {
    /// The incident team's attributes from a response.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IncidentTeamResponseAttributes>,
    /// The incident team's ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The incident team's relationships.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::IncidentTeamRelationships>,
    /// Incident Team resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::IncidentTeamType>,
}

impl IncidentTeamResponseData {
    pub fn new() -> IncidentTeamResponseData {
        IncidentTeamResponseData {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::IncidentTeamResponseAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_relationships(
        &mut self,
        value: crate::datadogV2::model::IncidentTeamRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: crate::datadogV2::model::IncidentTeamType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for IncidentTeamResponseData {
    fn default() -> Self {
        Self::new()
    }
}
