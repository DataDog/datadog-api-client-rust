// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTiming {
    /// The duration in millisecond of the DNS lookup.
    #[serde(rename = "dns", skip_serializing_if = "Option::is_none")]
    pub dns: f64,
    /// The time in millisecond to download the response.
    #[serde(rename = "download", skip_serializing_if = "Option::is_none")]
    pub download: f64,
    /// The time in millisecond to first byte.
    #[serde(rename = "firstByte", skip_serializing_if = "Option::is_none")]
    pub first_byte: f64,
    /// The duration in millisecond of the TLS handshake.
    #[serde(rename = "handshake", skip_serializing_if = "Option::is_none")]
    pub handshake: f64,
    /// The time in millisecond spent during redirections.
    #[serde(rename = "redirect", skip_serializing_if = "Option::is_none")]
    pub redirect: f64,
    /// The duration in millisecond of the TLS handshake.
    #[serde(rename = "ssl", skip_serializing_if = "Option::is_none")]
    pub ssl: f64,
    /// Time in millisecond to establish the TCP connection.
    #[serde(rename = "tcp", skip_serializing_if = "Option::is_none")]
    pub tcp: f64,
    /// The overall time in millisecond the request took to be processed.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: f64,
    /// Time spent in millisecond waiting for a response.
    #[serde(rename = "wait", skip_serializing_if = "Option::is_none")]
    pub wait: f64,
}

