// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringFilter {
    /// The type of filtering action.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: SecurityMonitoringFilterAction,
    /// Query for selecting logs to apply the filtering action.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
}

