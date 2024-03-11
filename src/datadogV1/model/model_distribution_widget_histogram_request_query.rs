// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Query definition for Distribution Widget Histogram Request
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for DistributionWidgetHistogramRequestQuery {
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
                return Ok(DistributionWidgetHistogramRequestQuery::FormulaAndFunctionMetricQueryDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionEventQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    DistributionWidgetHistogramRequestQuery::FormulaAndFunctionEventQueryDefinition(
                        _v,
                    ),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::FormulaAndFunctionApmResourceStatsQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(DistributionWidgetHistogramRequestQuery::FormulaAndFunctionApmResourceStatsQueryDefinition(_v));
            }
        }

        return Ok(DistributionWidgetHistogramRequestQuery::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
