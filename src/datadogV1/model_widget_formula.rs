// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetFormula {
    /// Expression alias.
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: String,
    /// Define a display mode for the table cell.
    #[serde(rename = "cell_display_mode", skip_serializing_if = "Option::is_none")]
    pub cell_display_mode: TableWidgetCellDisplayMode,
    /// List of conditional formats.
    #[serde(rename = "conditional_formats", skip_serializing_if = "Option::is_none")]
    pub conditional_formats: Vec<WidgetConditionalFormat>,
    /// String expression built from queries, formulas, and functions.
    #[serde(rename = "formula", skip_serializing_if = "Option::is_none")]
    pub formula: String,
    /// Options for limiting results returned.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: WidgetFormulaLimit,
    /// Styling options for widget formulas.
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: WidgetFormulaStyle,
}

