// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident integration metadata's attributes for a create request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentIntegrationMetadataAttributes {
    /// Timestamp when the incident todo was created.
    #[serde(rename = "created")]
    pub created: Option<String>,
    /// UUID of the incident this integration metadata is connected to.
    #[serde(rename = "incident_id")]
    pub incident_id: Option<String>,
    /// A number indicating the type of integration this metadata is for. 1 indicates Slack;
    /// 8 indicates Jira.
    #[serde(rename = "integration_type")]
    pub integration_type: i32,
    /// Incident integration metadata's metadata attribute.
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::datadogV2::model::IncidentIntegrationMetadataMetadata>,
    /// Timestamp when the incident todo was last modified.
    #[serde(rename = "modified")]
    pub modified: Option<String>,
    /// A number indicating the status of this integration metadata. 0 indicates unknown;
    /// 1 indicates pending; 2 indicates complete; 3 indicates manually created;
    /// 4 indicates manually updated; 5 indicates failed.
    #[serde(rename = "status")]
    pub status: Option<i32>,
}

impl IncidentIntegrationMetadataAttributes {
    pub fn new(
        integration_type: i32,
        metadata: Box<crate::datadogV2::model::IncidentIntegrationMetadataMetadata>,
    ) -> IncidentIntegrationMetadataAttributes {
        IncidentIntegrationMetadataAttributes {
            created: None,
            incident_id: None,
            integration_type,
            metadata,
            modified: None,
            status: None,
        }
    }
}
