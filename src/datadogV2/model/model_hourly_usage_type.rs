// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HourlyUsageType {
    #[serde(rename = "app_sec_host_count")]
    APP_SEC_HOST_COUNT,
    #[serde(rename = "observability_pipelines_bytes_processed")]
    OBSERVABILITY_PIPELINES_BYTES_PROCESSSED,
    #[serde(rename = "lambda_traced_invocations_count")]
    LAMBDA_TRACED_INVOCATIONS_COUNT,
}

impl ToString for HourlyUsageType {
    fn to_string(&self) -> String {
        match self {
            Self::APP_SEC_HOST_COUNT => String::from("app_sec_host_count"),
            Self::OBSERVABILITY_PIPELINES_BYTES_PROCESSSED => {
                String::from("observability_pipelines_bytes_processed")
            }
            Self::LAMBDA_TRACED_INVOCATIONS_COUNT => {
                String::from("lambda_traced_invocations_count")
            }
        }
    }
}

impl Default for HourlyUsageType {
    fn default() -> HourlyUsageType {
        Self::APP_SEC_HOST_COUNT
    }
}
