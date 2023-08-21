// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPRanges {
    /// Available prefix information for the Agent endpoints.
    #[serde(rename = "agents", skip_serializing_if = "Option::is_none")]
    pub agents: IPPrefixesAgents,
    /// Available prefix information for the API endpoints.
    #[serde(rename = "api", skip_serializing_if = "Option::is_none")]
    pub api: IPPrefixesAPI,
    /// Available prefix information for the APM endpoints.
    #[serde(rename = "apm", skip_serializing_if = "Option::is_none")]
    pub apm: IPPrefixesAPM,
    /// Available prefix information for the Logs endpoints.
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: IPPrefixesLogs,
    /// Date when last updated, in the form `YYYY-MM-DD-hh-mm-ss`.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: String,
    /// Available prefix information for the Orchestrator endpoints.
    #[serde(rename = "orchestrator", skip_serializing_if = "Option::is_none")]
    pub orchestrator: IPPrefixesOrchestrator,
    /// Available prefix information for the Process endpoints.
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    pub process: IPPrefixesProcess,
    /// Available prefix information for the Remote Configuration endpoints.
    #[serde(rename = "remote-configuration", skip_serializing_if = "Option::is_none")]
    pub remote_configuration: IPPrefixesRemoteConfiguration,
    /// Available prefix information for the Synthetics endpoints.
    #[serde(rename = "synthetics", skip_serializing_if = "Option::is_none")]
    pub synthetics: IPPrefixesSynthetics,
    /// Available prefix information for the Synthetics Private Locations endpoints.
    #[serde(rename = "synthetics-private-locations", skip_serializing_if = "Option::is_none")]
    pub synthetics_private_locations: IPPrefixesSyntheticsPrivateLocations,
    /// Version of the IP list.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: i64,
    /// Available prefix information for the Webhook endpoints.
    #[serde(rename = "webhooks", skip_serializing_if = "Option::is_none")]
    pub webhooks: IPPrefixesWebhooks,
}

