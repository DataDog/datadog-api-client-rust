// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SecurityMonitoringRuleTypeRead {
    #[serde(rename = "log_detection")]
    LOG_DETECTION,
    #[serde(rename = "infrastructure_configuration")]
    INFRASTRUCTURE_CONFIGURATION,
    #[serde(rename = "workload_security")]
    WORKLOAD_SECURITY,
    #[serde(rename = "cloud_configuration")]
    CLOUD_CONFIGURATION,
    #[serde(rename = "application_security")]
    APPLICATION_SECURITY,
}
impl ToString for SecurityMonitoringRuleTypeRead {
    fn to_string(&self) -> String {
        match self {
            Self::LOG_DETECTION => String::from("log_detection"),
            Self::INFRASTRUCTURE_CONFIGURATION => String::from("infrastructure_configuration"),
            Self::WORKLOAD_SECURITY => String::from("workload_security"),
            Self::CLOUD_CONFIGURATION => String::from("cloud_configuration"),
            Self::APPLICATION_SECURITY => String::from("application_security"),
        }
    }
}
