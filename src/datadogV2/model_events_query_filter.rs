// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsQueryFilter {
    /// The minimum time for the requested events. Supports date math and regular timestamps in milliseconds.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: String,
    /// The search query following the event search syntax.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: String,
    /// The maximum time for the requested events. Supports date math and regular timestamps in milliseconds.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: String,
}

