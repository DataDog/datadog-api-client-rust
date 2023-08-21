// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostTotals {
    /// Total number of active host (UP and ???) reporting to Datadog.
    #[serde(rename = "total_active", skip_serializing_if = "Option::is_none")]
    pub total_active: i64,
    /// Number of host that are UP and reporting to Datadog.
    #[serde(rename = "total_up", skip_serializing_if = "Option::is_none")]
    pub total_up: i64,
}

