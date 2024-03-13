// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A formula for calculation based on one or more queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFormula {
    /// Formula string, referencing one or more queries with their name property.
    #[serde(rename = "formula")]
    pub formula: String,
    /// Message for specifying limits to the number of values returned by a query.
    /// This limit is only for scalar queries and has no effect on timeseries queries.
    #[serde(rename = "limit")]
    pub limit: Option<crate::datadogV2::model::FormulaLimit>,
}

impl QueryFormula {
    pub fn new(formula: String) -> QueryFormula {
        QueryFormula {
            formula,
            limit: None,
        }
    }

    pub fn limit(mut self, value: crate::datadogV2::model::FormulaLimit) -> Self {
        self.limit = Some(value);
        self
    }
}
