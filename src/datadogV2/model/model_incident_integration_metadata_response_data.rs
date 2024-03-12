// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident integration metadata from a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentIntegrationMetadataResponseData {
    /// Incident integration metadata's attributes for a create request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IncidentIntegrationMetadataAttributes>,
    /// The incident integration metadata's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The incident's integration relationships from a response.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::IncidentIntegrationRelationships>,
    /// Integration metadata resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentIntegrationMetadataType,
}

impl IncidentIntegrationMetadataResponseData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::IncidentIntegrationMetadataType,
    ) -> IncidentIntegrationMetadataResponseData {
        IncidentIntegrationMetadataResponseData {
            attributes: None,
            id,
            relationships: None,
            type_,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::IncidentIntegrationMetadataAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::IncidentIntegrationRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }
}
