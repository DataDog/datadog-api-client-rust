// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Measure and calculation settings for a numerator or denominator aggregation. Supply exactly one non-null measure.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation {
    ExperimentsWarehouseMetricAggregationInput(
        Box<crate::datadogV2::model::ExperimentsWarehouseMetricAggregationInput>,
    ),
    ExperimentsDatadogMetricAggregationInput(
        Box<crate::datadogV2::model::ExperimentsDatadogMetricAggregationInput>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsWarehouseMetricAggregationInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation::ExperimentsWarehouseMetricAggregationInput(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsDatadogMetricAggregationInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation::ExperimentsDatadogMetricAggregationInput(_v));
            }
        }

        return Ok(
            ExperimentsCreateMetricV2RequestDataAttributesNumeratorAggregation::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
