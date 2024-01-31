// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident Service payload for create requests.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentServiceCreateData {
    /// The incident service's attributes for a create request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IncidentServiceCreateAttributes>,
    /// The incident service's relationships.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::IncidentServiceRelationships>,
    /// Incident service resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentServiceType,
}

impl IncidentServiceCreateData {
    pub fn new(type_: crate::datadogV2::model::IncidentServiceType) -> IncidentServiceCreateData {
        IncidentServiceCreateData {
            attributes: None,
            relationships: None,
            type_,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::IncidentServiceCreateAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_relationships(
        &mut self,
        value: crate::datadogV2::model::IncidentServiceRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }
}
