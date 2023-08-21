// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseMetaPage {
    /// The first number.
    #[serde(rename = "first_number", skip_serializing_if = "Option::is_none")]
    pub first_number: i64,
    /// The last number.
    #[serde(rename = "last_number", skip_serializing_if = "Option::is_none")]
    pub last_number: i64,
    /// The next number.
    #[serde(rename = "next_number", skip_serializing_if = "Option::is_none")]
    pub next_number: i64,
    /// The page number.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: i64,
    /// The previous page number.
    #[serde(rename = "prev_number", skip_serializing_if = "Option::is_none")]
    pub prev_number: i64,
    /// The size of the response.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: i64,
    /// The total number of SLOs in the response.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: i64,
    /// Type of pagination.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
}

