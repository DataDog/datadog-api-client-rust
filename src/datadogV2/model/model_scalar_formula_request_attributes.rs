// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object describing a scalar formula request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarFormulaRequestAttributes {
    /// List of formulas to be calculated and returned as responses.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV2::model::QueryFormula>>,
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// List of queries to be run and used as inputs to the formulas.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::ScalarQuery>,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to")]
    pub to: i64,
}

impl ScalarFormulaRequestAttributes {
    pub fn new(
        from: i64,
        queries: Vec<crate::datadogV2::model::ScalarQuery>,
        to: i64,
    ) -> ScalarFormulaRequestAttributes {
        ScalarFormulaRequestAttributes {
            formulas: None,
            from,
            queries,
            to,
        }
    }

    pub fn formulas(&mut self, value: Vec<crate::datadogV2::model::QueryFormula>) -> &mut Self {
        self.formulas = Some(value);
        self
    }
}
