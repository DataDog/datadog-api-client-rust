// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleTypeCreate {
    #[serde(rename = "application_security")]
    APPLICATION_SECURITY,
    #[serde(rename = "log_detection")]
    LOG_DETECTION,
    #[serde(rename = "workload_security")]
    WORKLOAD_SECURITY,
}

impl ToString for SecurityMonitoringRuleTypeCreate {
    fn to_string(&self) -> String {
        match self {
            Self::APPLICATION_SECURITY => String::from("application_security"),
            Self::LOG_DETECTION => String::from("log_detection"),
            Self::WORKLOAD_SECURITY => String::from("workload_security"),
        }
    }
}
