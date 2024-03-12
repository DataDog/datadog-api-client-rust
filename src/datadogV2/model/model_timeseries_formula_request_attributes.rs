// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object describing a timeseries formula request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesFormulaRequestAttributes {
    /// List of formulas to be calculated and returned as responses.
    #[serde(rename = "formulas")]
    pub formulas: Option<Vec<crate::datadogV2::model::QueryFormula>>,
    /// Start date (inclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// A time interval in milliseconds.
    /// May be overridden by a larger interval if the query would result in
    /// too many points for the specified timeframe.
    /// Defaults to a reasonable interval for the given timeframe.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// List of queries to be run and used as inputs to the formulas.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::TimeseriesQuery>,
    /// End date (exclusive) of the query in milliseconds since the Unix epoch.
    #[serde(rename = "to")]
    pub to: i64,
}

impl TimeseriesFormulaRequestAttributes {
    pub fn new(
        from: i64,
        queries: Vec<crate::datadogV2::model::TimeseriesQuery>,
        to: i64,
    ) -> TimeseriesFormulaRequestAttributes {
        TimeseriesFormulaRequestAttributes {
            formulas: None,
            from,
            interval: None,
            queries,
            to,
        }
    }

    pub fn formulas(mut self, value: Vec<crate::datadogV2::model::QueryFormula>) -> Self {
        self.formulas = Some(value);
        self
    }

    pub fn interval(mut self, value: i64) -> Self {
        self.interval = Some(value);
        self
    }
}
