// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A formula and function query.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for FormulaAndFunctionQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionMetricQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    FormulaAndFunctionQueryDefinition::FormulaAndFunctionMetricQueryDefinition(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionEventQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    FormulaAndFunctionQueryDefinition::FormulaAndFunctionEventQueryDefinition(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionProcessQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    FormulaAndFunctionQueryDefinition::FormulaAndFunctionProcessQueryDefinition(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionApmDependencyStatsQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaAndFunctionApmDependencyStatsQueryDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionApmResourceStatsQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(FormulaAndFunctionQueryDefinition::FormulaAndFunctionApmResourceStatsQueryDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionSLOQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    FormulaAndFunctionQueryDefinition::FormulaAndFunctionSLOQueryDefinition(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionCloudCostQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    FormulaAndFunctionQueryDefinition::FormulaAndFunctionCloudCostQueryDefinition(
                        _v,
                    ),
                );
            }
        }

        return Ok(FormulaAndFunctionQueryDefinition::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
