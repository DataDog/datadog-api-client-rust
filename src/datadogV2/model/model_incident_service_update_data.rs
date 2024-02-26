// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident Service payload for update requests.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentServiceUpdateData {
    /// The incident service's attributes for an update request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IncidentServiceUpdateAttributes>,
    /// The incident service's ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The incident service's relationships.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::IncidentServiceRelationships>,
    /// Incident service resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentServiceType,
}

impl IncidentServiceUpdateData {
    pub fn new(type_: crate::datadogV2::model::IncidentServiceType) -> IncidentServiceUpdateData {
        IncidentServiceUpdateData {
            attributes: None,
            id: None,
            relationships: None,
            type_,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::IncidentServiceUpdateAttributes,
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
        value: crate::datadogV2::model::IncidentServiceRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }
}
