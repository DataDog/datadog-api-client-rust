// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetFieldSort {
    /// Facet path for the column
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: String,
    /// Widget sorting methods.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: WidgetSort,
}

