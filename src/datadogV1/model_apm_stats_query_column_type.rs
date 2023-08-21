// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApmStatsQueryColumnType {
    /// A user-assigned alias for the column.
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: String,
    /// Define a display mode for the table cell.
    #[serde(rename = "cell_display_mode", skip_serializing_if = "Option::is_none")]
    pub cell_display_mode: TableWidgetCellDisplayMode,
    /// Column name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Widget sorting methods.
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: WidgetSort,
}

