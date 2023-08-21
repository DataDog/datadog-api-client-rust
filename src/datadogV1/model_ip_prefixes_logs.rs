// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPPrefixesLogs {
    /// List of IPv4 prefixes.
    #[serde(rename = "prefixes_ipv4", skip_serializing_if = "Option::is_none")]
    pub prefixes_ipv4: Vec<String>,
    /// List of IPv6 prefixes.
    #[serde(rename = "prefixes_ipv6", skip_serializing_if = "Option::is_none")]
    pub prefixes_ipv6: Vec<String>,
}

