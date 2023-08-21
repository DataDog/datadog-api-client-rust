// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOResponseLinks {
    /// Link to last page.
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: String,
    /// Link to first page.
    #[serde(rename = "last", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub last: Option<String>,
    /// Link to the next page.
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: String,
    /// Link to previous page.
    #[serde(rename = "prev", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub prev: Option<String>,
    /// Link to current page.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: String,
}

