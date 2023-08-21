// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentResponseMetaPagination {
    /// The index of the first element in the next page of results. Equal to page size added to the current offset.
    #[serde(rename = "next_offset", skip_serializing_if = "Option::is_none")]
    pub next_offset: i64,
    /// The index of the first element in the results.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: i64,
    /// Maximum size of pages to return.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: i64,
}

