// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetFormulaStyle {
    /// The color palette used to display the formula. A guide to the available color palettes can be found at https://docs.datadoghq.com/dashboards/guide/widget_colors
    #[serde(rename = "palette", skip_serializing_if = "Option::is_none")]
    pub palette: String,
    /// Index specifying which color to use within the palette.
    #[serde(rename = "palette_index", skip_serializing_if = "Option::is_none")]
    pub palette_index: i64,
}

