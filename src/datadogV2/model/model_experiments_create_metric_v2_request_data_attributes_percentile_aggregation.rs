// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Measure and percentile to calculate for the metric. Supply exactly one non-null measure.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation {
    ExperimentsWarehousePercentileAggregationInput(
        Box<crate::datadogV2::model::ExperimentsWarehousePercentileAggregationInput>,
    ),
    ExperimentsDatadogPercentileAggregationInput(
        Box<crate::datadogV2::model::ExperimentsDatadogPercentileAggregationInput>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsWarehousePercentileAggregationInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation::ExperimentsWarehousePercentileAggregationInput(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsDatadogPercentileAggregationInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation::ExperimentsDatadogPercentileAggregationInput(_v));
            }
        }

        return Ok(
            ExperimentsCreateMetricV2RequestDataAttributesPercentileAggregation::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
