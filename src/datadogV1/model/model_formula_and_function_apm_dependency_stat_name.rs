// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FormulaAndFunctionApmDependencyStatName {
    #[serde(rename = "avg_duration")]
    AVG_DURATION,
    #[serde(rename = "avg_root_duration")]
    AVG_ROOT_DURATION,
    #[serde(rename = "avg_spans_per_trace")]
    AVG_SPANS_PER_TRACE,
    #[serde(rename = "error_rate")]
    ERROR_RATE,
    #[serde(rename = "pct_exec_time")]
    PCT_EXEC_TIME,
    #[serde(rename = "pct_of_traces")]
    PCT_OF_TRACES,
    #[serde(rename = "total_traces_count")]
    TOTAL_TRACES_COUNT,
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
        }
    }
}
