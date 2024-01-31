// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// A formula and function query.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormulaAndFunctionQueryDefinition {
    FormulaAndFunctionMetricQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionMetricQueryDefinition,
    ),
    FormulaAndFunctionEventQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionEventQueryDefinition,
    ),
    FormulaAndFunctionProcessQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionProcessQueryDefinition,
    ),
    FormulaAndFunctionApmDependencyStatsQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsQueryDefinition,
    ),
    FormulaAndFunctionApmResourceStatsQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionApmResourceStatsQueryDefinition,
    ),
    FormulaAndFunctionSLOQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionSLOQueryDefinition,
    ),
    FormulaAndFunctionCloudCostQueryDefinition(
        crate::datadogV1::model::FormulaAndFunctionCloudCostQueryDefinition,
    ),
}
