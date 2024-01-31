// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Query definition for Distribution Widget Histogram Request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DistributionWidgetHistogramRequestQuery {
    FormulaAndFunctionMetricQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionMetricQueryDefinition,
    ),
    FormulaAndFunctionEventQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionEventQueryDefinition,
    ),
    FormulaAndFunctionApmResourceStatsQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionApmResourceStatsQueryDefinition,
    ),
}
