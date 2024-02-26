// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// A formula and function query.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormulaAndFunctionQueryDefinition {
    FormulaAndFunctionMetricQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionMetricQueryDefinition>,
    ),
    FormulaAndFunctionEventQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionEventQueryDefinition>,
    ),
    FormulaAndFunctionProcessQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionProcessQueryDefinition>,
    ),
    FormulaAndFunctionApmDependencyStatsQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsQueryDefinition>,
    ),
    FormulaAndFunctionApmResourceStatsQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionApmResourceStatsQueryDefinition>,
    ),
    FormulaAndFunctionSLOQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionSLOQueryDefinition>,
    ),
    FormulaAndFunctionCloudCostQueryDefinition(
        Box<crate::datadogV1::model::FormulaAndFunctionCloudCostQueryDefinition>,
    ),
}
