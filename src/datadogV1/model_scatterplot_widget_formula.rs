// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterplotWidgetFormula {
    /// Expression alias.
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: String,
    /// Dimension of the Scatterplot.
    #[serde(rename = "dimension", skip_serializing_if = "Option::is_none")]
    pub dimension: ScatterplotDimension,
    /// String expression built from queries, formulas, and functions.
    #[serde(rename = "formula", skip_serializing_if = "Option::is_none")]
    pub formula: String,
}

