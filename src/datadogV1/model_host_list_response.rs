// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostListResponse {
    /// Array of hosts.
    #[serde(rename = "host_list", skip_serializing_if = "Option::is_none")]
    pub host_list: Vec<Host>,
    /// Number of host matching the query.
    #[serde(rename = "total_matching", skip_serializing_if = "Option::is_none")]
    pub total_matching: i64,
    /// Number of host returned.
    #[serde(rename = "total_returned", skip_serializing_if = "Option::is_none")]
    pub total_returned: i64,
}

