// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with an incident integration metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentIntegrationMetadataResponse {
    /// Incident integration metadata from a response.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::IncidentIntegrationMetadataResponseData,
    /// Included related resources that the user requested.
    #[serde(rename = "included")]
    pub included:
        Option<Vec<crate::datadogV2::model::IncidentIntegrationMetadataResponseIncludedItem>>,
}

impl IncidentIntegrationMetadataResponse {
    pub fn new(
        data: crate::datadogV2::model::IncidentIntegrationMetadataResponseData,
    ) -> IncidentIntegrationMetadataResponse {
        IncidentIntegrationMetadataResponse {
            data,
            included: None,
        }
    }

    pub fn with_included(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentIntegrationMetadataResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }
}
