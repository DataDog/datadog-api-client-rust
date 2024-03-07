// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FormulaAndFunctionApmResourceStatName {
    #[serde(rename = "errors")]
    ERRORS,
    #[serde(rename = "error_rate")]
    ERROR_RATE,
    #[serde(rename = "hits")]
    HITS,
    #[serde(rename = "latency_avg")]
    LATENCY_AVG,
    #[serde(rename = "latency_distribution")]
    LATENCY_DISTRIBUTION,
    #[serde(rename = "latency_max")]
    LATENCY_MAX,
    #[serde(rename = "latency_p50")]
    LATENCY_P50,
    #[serde(rename = "latency_p75")]
    LATENCY_P75,
    #[serde(rename = "latency_p90")]
    LATENCY_P90,
    #[serde(rename = "latency_p95")]
    LATENCY_P95,
    #[serde(rename = "latency_p99")]
    LATENCY_P99,
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
        }
    }
}
