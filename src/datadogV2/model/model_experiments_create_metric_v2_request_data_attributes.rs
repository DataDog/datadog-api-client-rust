// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Configuration for the new metric. Supply either numerator_aggregation or percentile_aggregation. A denominator_aggregation requires numerator_aggregation. Omit unused aggregation fields; do not send them as null.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExperimentsCreateMetricV2RequestDataAttributes {
    ExperimentsCreateMetricNumeratorAttributes(
        Box<crate::datadogV2::model::ExperimentsCreateMetricNumeratorAttributes>,
    ),
    ExperimentsCreateMetricPercentileAttributes(
        Box<crate::datadogV2::model::ExperimentsCreateMetricPercentileAttributes>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ExperimentsCreateMetricV2RequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsCreateMetricNumeratorAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsCreateMetricV2RequestDataAttributes::ExperimentsCreateMetricNumeratorAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsCreateMetricPercentileAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsCreateMetricV2RequestDataAttributes::ExperimentsCreateMetricPercentileAttributes(_v));
            }
        }

        return Ok(
            ExperimentsCreateMetricV2RequestDataAttributes::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
