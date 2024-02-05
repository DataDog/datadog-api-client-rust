// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A relationship reference for an integration metadata object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToIncidentIntegrationMetadataData {
    /// A unique identifier that represents the integration metadata.
    #[serde(rename = "id")]
    pub id: String,
    /// Integration metadata resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentIntegrationMetadataType,
}

impl RelationshipToIncidentIntegrationMetadataData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::IncidentIntegrationMetadataType,
    ) -> RelationshipToIncidentIntegrationMetadataData {
        RelationshipToIncidentIntegrationMetadataData { id, type_ }
    }
}
