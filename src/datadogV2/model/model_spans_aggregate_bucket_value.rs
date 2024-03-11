// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A bucket value, can be either a timeseries or a single value.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SpansAggregateBucketValue {
    SpansAggregateBucketValueSingleString(String),
    SpansAggregateBucketValueSingleNumber(f64),
    SpansAggregateBucketValueTimeseries(
        Vec<crate::datadogV2::model::SpansAggregateBucketValueTimeseriesPoint>,
    ),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for SpansAggregateBucketValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(SpansAggregateBucketValue::SpansAggregateBucketValueSingleString(_v));
        }
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(SpansAggregateBucketValue::SpansAggregateBucketValueSingleNumber(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Vec<crate::datadogV2::model::SpansAggregateBucketValueTimeseriesPoint>,
        >(value.clone())
        {
            return Ok(SpansAggregateBucketValue::SpansAggregateBucketValueTimeseries(_v));
        }

        return Ok(SpansAggregateBucketValue::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
