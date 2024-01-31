// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Formula to be used in a Scatterplot widget query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterplotWidgetFormula {
    /// Expression alias.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Dimension of the Scatterplot.
    #[serde(rename = "dimension")]
    pub dimension: crate::datadogV1::model::ScatterplotDimension,
    /// String expression built from queries, formulas, and functions.
    #[serde(rename = "formula")]
    pub formula: String,
}

impl ScatterplotWidgetFormula {
    pub fn new(
        dimension: crate::datadogV1::model::ScatterplotDimension,
        formula: String,
    ) -> ScatterplotWidgetFormula {
        ScatterplotWidgetFormula {
            alias: None,
            dimension,
            formula,
        }
    }

    pub fn with_alias(&mut self, value: String) -> &mut Self {
        self.alias = Some(value);
        self
    }
}
