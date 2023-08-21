// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOListResponseMetadataPage {
    /// The total number of resources that could be retrieved ignoring the parameters and filters in the request.
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: i64,
    /// The total number of resources that match the parameters and filters in the request. This attribute can be used by a client to determine the total number of pages.
    #[serde(rename = "total_filtered_count", skip_serializing_if = "Option::is_none")]
    pub total_filtered_count: i64,
}

