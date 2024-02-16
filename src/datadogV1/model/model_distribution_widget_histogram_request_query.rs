// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Query definition for Distribution Widget Histogram Request
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DistributionWidgetHistogramRequestQuery {
    FormulaAndFunctionMetricQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionMetricQueryDefinition>,
    ),
    FormulaAndFunctionEventQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionEventQueryDefinition>,
    ),
    FormulaAndFunctionApmResourceStatsQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionApmResourceStatsQueryDefinition>,
    ),
}
