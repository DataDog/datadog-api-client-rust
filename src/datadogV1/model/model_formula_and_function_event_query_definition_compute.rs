// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Compute options.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionEventQueryDefinitionCompute {
    /// Aggregation methods for event platform queries.
    #[serde(rename = "aggregation")]
    pub aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    /// A time interval in milliseconds.
    #[serde(rename = "interval")]
    pub interval: Option<i64>,
    /// Measurable attribute to compute.
    #[serde(rename = "metric")]
    pub metric: Option<String>,
}

impl FormulaAndFunctionEventQueryDefinitionCompute {
    pub fn new(
        aggregation: crate::datadogV1::model::FormulaAndFunctionEventAggregation,
    ) -> FormulaAndFunctionEventQueryDefinitionCompute {
        FormulaAndFunctionEventQueryDefinitionCompute {
            aggregation,
            interval: None,
            metric: None,
        }
    }
}