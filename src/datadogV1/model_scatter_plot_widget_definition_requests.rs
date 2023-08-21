// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterPlotWidgetDefinitionRequests {
    /// Scatterplot request containing formulas and functions.
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: ScatterplotTableRequest,
    /// Updated scatter plot.
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: ScatterPlotRequest,
    /// Updated scatter plot.
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: ScatterPlotRequest,
}

