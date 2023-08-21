// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsRequestPage {
    /// The returned paging point to use to get the next results.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: String,
    /// The maximum number of logs in the response.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i32,
}

