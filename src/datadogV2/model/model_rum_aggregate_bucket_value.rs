// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A bucket value, can be either a timeseries or a single value.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RUMAggregateBucketValue {
    RUMAggregateBucketValueSingleString(String),
    RUMAggregateBucketValueSingleNumber(f64),
    RUMAggregateBucketValueTimeseries(
        Vec<crate::datadogV2::model::RUMAggregateBucketValueTimeseriesPoint>,
    ),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for RUMAggregateBucketValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(RUMAggregateBucketValue::RUMAggregateBucketValueSingleString(_v));
        }
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(RUMAggregateBucketValue::RUMAggregateBucketValueSingleNumber(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Vec<crate::datadogV2::model::RUMAggregateBucketValueTimeseriesPoint>,
        >(value.clone())
        {
            return Ok(RUMAggregateBucketValue::RUMAggregateBucketValueTimeseries(
                _v,
            ));
        }

        return Ok(RUMAggregateBucketValue::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
