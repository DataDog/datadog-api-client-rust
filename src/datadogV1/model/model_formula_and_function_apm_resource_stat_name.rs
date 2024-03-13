// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaAndFunctionApmResourceStatName {
    ERRORS,
    ERROR_RATE,
    HITS,
    LATENCY_AVG,
    LATENCY_DISTRIBUTION,
    LATENCY_MAX,
    LATENCY_P50,
    LATENCY_P75,
    LATENCY_P90,
    LATENCY_P95,
    LATENCY_P99,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FormulaAndFunctionApmResourceStatName {
    fn to_string(&self) -> String {
        match self {
            Self::ERRORS => String::from("errors"),
            Self::ERROR_RATE => String::from("error_rate"),
            Self::HITS => String::from("hits"),
            Self::LATENCY_AVG => String::from("latency_avg"),
            Self::LATENCY_DISTRIBUTION => String::from("latency_distribution"),
            Self::LATENCY_MAX => String::from("latency_max"),
            Self::LATENCY_P50 => String::from("latency_p50"),
            Self::LATENCY_P75 => String::from("latency_p75"),
            Self::LATENCY_P90 => String::from("latency_p90"),
            Self::LATENCY_P95 => String::from("latency_p95"),
            Self::LATENCY_P99 => String::from("latency_p99"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FormulaAndFunctionApmResourceStatName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for FormulaAndFunctionApmResourceStatName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "errors" => Self::ERRORS,
            "error_rate" => Self::ERROR_RATE,
            "hits" => Self::HITS,
            "latency_avg" => Self::LATENCY_AVG,
            "latency_distribution" => Self::LATENCY_DISTRIBUTION,
            "latency_max" => Self::LATENCY_MAX,
            "latency_p50" => Self::LATENCY_P50,
            "latency_p75" => Self::LATENCY_P75,
            "latency_p90" => Self::LATENCY_P90,
            "latency_p95" => Self::LATENCY_P95,
            "latency_p99" => Self::LATENCY_P99,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
