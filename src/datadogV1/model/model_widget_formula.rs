// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Formula to be used in a widget query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetFormula {
    /// Expression alias.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Define a display mode for the table cell.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<crate::datadogV1::model::TableWidgetCellDisplayMode>,
    /// List of conditional formats.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::WidgetConditionalFormat>>,
    /// String expression built from queries, formulas, and functions.
    #[serde(rename = "formula")]
    pub formula: String,
    /// Options for limiting results returned.
    #[serde(rename = "limit")]
    pub limit: Option<Box<crate::datadogV1::model::WidgetFormulaLimit>>,
    /// Styling options for widget formulas.
    #[serde(rename = "style")]
    pub style: Option<Box<crate::datadogV1::model::WidgetFormulaStyle>>,
}

impl WidgetFormula {
    pub fn new(formula: String) -> WidgetFormula {
        WidgetFormula {
            alias: None,
            cell_display_mode: None,
            conditional_formats: None,
            formula,
            limit: None,
            style: None,
        }
    }
}