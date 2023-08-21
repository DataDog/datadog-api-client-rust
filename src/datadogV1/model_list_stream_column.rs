// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListStreamColumn {
    /// Widget column field.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: String,
    /// Widget column width.
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: ListStreamColumnWidth,
}

