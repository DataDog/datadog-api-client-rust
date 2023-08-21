// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCanDeleteSLOResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: CheckCanDeleteSLOResponseData,
    /// A mapping of SLO id to it's current usages.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: map[string]String,
}

