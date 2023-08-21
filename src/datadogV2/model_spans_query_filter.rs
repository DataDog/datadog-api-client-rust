// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansQueryFilter {
    /// The minimum time for the requested spans, supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: String,
    /// The search query - following the span search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// The maximum time for the requested spans, supports date-time ISO8601, date math, and regular timestamps (milliseconds).
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: String,
}

