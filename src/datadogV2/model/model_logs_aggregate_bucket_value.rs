// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A bucket value, can be either a timeseries or a single value
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum LogsAggregateBucketValue {
    LogsAggregateBucketValueSingleString(String),
    LogsAggregateBucketValueSingleNumber(f64),
    LogsAggregateBucketValueTimeseries(
        Vec<crate::datadogV2::model::LogsAggregateBucketValueTimeseriesPoint>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for LogsAggregateBucketValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<String>(value.clone()) {
            return Ok(LogsAggregateBucketValue::LogsAggregateBucketValueSingleString(_v));
        }
        if let Ok(_v) = serde_json::from_value::<f64>(value.clone()) {
            return Ok(LogsAggregateBucketValue::LogsAggregateBucketValueSingleNumber(_v));
        }
        if let Ok(_v) = serde_json::from_value::<
            Vec<crate::datadogV2::model::LogsAggregateBucketValueTimeseriesPoint>,
        >(value.clone())
        {
            return Ok(LogsAggregateBucketValue::LogsAggregateBucketValueTimeseries(_v));
        }

        return Ok(LogsAggregateBucketValue::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
