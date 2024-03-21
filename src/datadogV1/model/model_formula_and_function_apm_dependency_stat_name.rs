// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaAndFunctionApmDependencyStatName {
    AVG_DURATION,
    AVG_ROOT_DURATION,
    AVG_SPANS_PER_TRACE,
    ERROR_RATE,
    PCT_EXEC_TIME,
    PCT_OF_TRACES,
    TOTAL_TRACES_COUNT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FormulaAndFunctionApmDependencyStatName {
    fn to_string(&self) -> String {
        match self {
            Self::AVG_DURATION => String::from("avg_duration"),
            Self::AVG_ROOT_DURATION => String::from("avg_root_duration"),
            Self::AVG_SPANS_PER_TRACE => String::from("avg_spans_per_trace"),
            Self::ERROR_RATE => String::from("error_rate"),
            Self::PCT_EXEC_TIME => String::from("pct_exec_time"),
            Self::PCT_OF_TRACES => String::from("pct_of_traces"),
            Self::TOTAL_TRACES_COUNT => String::from("total_traces_count"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FormulaAndFunctionApmDependencyStatName {
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

impl<'de> Deserialize<'de> for FormulaAndFunctionApmDependencyStatName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg_duration" => Self::AVG_DURATION,
            "avg_root_duration" => Self::AVG_ROOT_DURATION,
            "avg_spans_per_trace" => Self::AVG_SPANS_PER_TRACE,
            "error_rate" => Self::ERROR_RATE,
            "pct_exec_time" => Self::PCT_EXEC_TIME,
            "pct_of_traces" => Self::PCT_OF_TRACES,
            "total_traces_count" => Self::TOTAL_TRACES_COUNT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
