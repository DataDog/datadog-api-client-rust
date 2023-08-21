// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsResponseMetaPagination {
    /// The first offset.
    #[serde(rename = "first_offset", skip_serializing_if = "Option::is_none")]
    pub first_offset: i64,
    /// The last offset.
    #[serde(rename = "last_offset", skip_serializing_if = "Option::is_none")]
    pub last_offset: i64,
    /// Pagination limit.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: i64,
    /// The next offset.
    #[serde(rename = "next_offset", skip_serializing_if = "Option::is_none")]
    pub next_offset: i64,
    /// The offset.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: i64,
    /// The previous offset.
    #[serde(rename = "prev_offset", skip_serializing_if = "Option::is_none")]
    pub prev_offset: i64,
    /// Total results.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: i64,
    /// Offset type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}

