// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
/// to be used because this will sum up all request counts instead of averaging them, or taking the max or
/// min of all of those requests.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchSLOQuery {
    /// A Datadog metric query for total (valid) events.
    #[serde(rename = "denominator")]
    pub denominator: Option<String>,
    /// Metric names used in the query's numerator and denominator.
    /// This field will return null and will be implemented in the next version of this endpoint.
    #[serde(
        rename = "metrics",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub metrics: Option<Option<Vec<String>>>,
    /// A Datadog metric query for good events.
    #[serde(rename = "numerator")]
    pub numerator: Option<String>,
}

impl SearchSLOQuery {
    pub fn new() -> SearchSLOQuery {
        SearchSLOQuery {
            denominator: None,
            metrics: None,
            numerator: None,
        }
    }

    pub fn denominator(&mut self, value: String) -> &mut Self {
        self.denominator = Some(value);
        self
    }

    pub fn metrics(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.metrics = Some(value);
        self
    }

    pub fn numerator(&mut self, value: String) -> &mut Self {
        self.numerator = Some(value);
        self
    }
}

impl Default for SearchSLOQuery {
    fn default() -> Self {
        Self::new()
    }
}
