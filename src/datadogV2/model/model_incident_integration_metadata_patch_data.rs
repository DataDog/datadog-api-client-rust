// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident integration metadata data for a patch request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentIntegrationMetadataPatchData {
    /// Incident integration metadata's attributes for a create request.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::IncidentIntegrationMetadataAttributes>,
    /// Integration metadata resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentIntegrationMetadataType,
}

impl IncidentIntegrationMetadataPatchData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::IncidentIntegrationMetadataAttributes>,
        type_: crate::datadogV2::model::IncidentIntegrationMetadataType,
    ) -> IncidentIntegrationMetadataPatchData {
        IncidentIntegrationMetadataPatchData { attributes, type_ }
    }
}
