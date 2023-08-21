// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLODeleteResponse {
    /// An array containing the ID of the deleted service level objective object.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Vec<String>,
    /// An dictionary containing the ID of the SLO as key and a deletion error as value.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: map[string]String,
}

