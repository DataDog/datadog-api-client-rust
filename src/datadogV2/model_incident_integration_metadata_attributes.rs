// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentIntegrationMetadataAttributes {
    /// UUID of the incident this integration metadata is connected to.
    #[serde(rename = "incident_id", skip_serializing_if = "Option::is_none")]
    pub incident_id: String,
    /// A number indicating the type of integration this metadata is for. 1 indicates Slack;
8 indicates Jira.
    #[serde(rename = "integration_type", skip_serializing_if = "Option::is_none")]
    pub integration_type: i32,
    /// Incident integration metadata's metadata attribute.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: IncidentIntegrationMetadataMetadata,
    /// A number indicating the status of this integration metadata. 0 indicates unknown;
1 indicates pending; 2 indicates complete; 3 indicates manually created;
4 indicates manually updated; 5 indicates failed.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: i32,
}

