// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident integration metadata data for a create request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentIntegrationMetadataCreateData {
    /// Incident integration metadata's attributes for a create request.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::IncidentIntegrationMetadataAttributes,
    /// Integration metadata resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentIntegrationMetadataType,
}

impl IncidentIntegrationMetadataCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::IncidentIntegrationMetadataAttributes,
        type_: crate::datadogV2::model::IncidentIntegrationMetadataType,
    ) -> IncidentIntegrationMetadataCreateData {
        IncidentIntegrationMetadataCreateData { attributes, type_ }
    }
}
