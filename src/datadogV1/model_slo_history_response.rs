// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: SLOHistoryResponseData,
    /// A list of errors while querying the history data for the service level objective.
    #[serde(rename = "errors", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub errors: Vec<SLOHistoryResponseError>,
}

