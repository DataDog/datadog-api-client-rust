// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOResponse {
    /// A service level objective object includes a service level indicator, thresholds
for one or more timeframes, and metadata (`name`, `description`, `tags`, etc.).
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: SLOResponseData,
    /// An array of error messages. Each endpoint documents how/whether this field is
used.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Vec<String>,
}

