// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonitorType {
    #[serde(rename = "composite")]
    COMPOSITE,
    #[serde(rename = "event alert")]
    EVENT_ALERT,
    #[serde(rename = "log alert")]
    LOG_ALERT,
    #[serde(rename = "metric alert")]
    METRIC_ALERT,
    #[serde(rename = "process alert")]
    PROCESS_ALERT,
    #[serde(rename = "query alert")]
    QUERY_ALERT,
    #[serde(rename = "rum alert")]
    RUM_ALERT,
    #[serde(rename = "service check")]
    SERVICE_CHECK,
    #[serde(rename = "synthetics alert")]
    SYNTHETICS_ALERT,
    #[serde(rename = "trace-analytics alert")]
    TRACE_ANALYTICS_ALERT,
    #[serde(rename = "slo alert")]
    SLO_ALERT,
    #[serde(rename = "event-v2 alert")]
    EVENT_V2_ALERT,
    #[serde(rename = "audit alert")]
    AUDIT_ALERT,
    #[serde(rename = "ci-pipelines alert")]
    CI_PIPELINES_ALERT,
    #[serde(rename = "ci-tests alert")]
    CI_TESTS_ALERT,
    #[serde(rename = "error-tracking alert")]
    ERROR_TRACKING_ALERT,
    #[serde(rename = "database-monitoring alert")]
    DATABASE_MONITORING_ALERT,
}

impl ToString for MonitorType {
    fn to_string(&self) -> String {
        match self {
            Self::COMPOSITE => String::from("composite"),
            Self::EVENT_ALERT => String::from("event alert"),
            Self::LOG_ALERT => String::from("log alert"),
            Self::METRIC_ALERT => String::from("metric alert"),
            Self::PROCESS_ALERT => String::from("process alert"),
            Self::QUERY_ALERT => String::from("query alert"),
            Self::RUM_ALERT => String::from("rum alert"),
            Self::SERVICE_CHECK => String::from("service check"),
            Self::SYNTHETICS_ALERT => String::from("synthetics alert"),
            Self::TRACE_ANALYTICS_ALERT => String::from("trace-analytics alert"),
            Self::SLO_ALERT => String::from("slo alert"),
            Self::EVENT_V2_ALERT => String::from("event-v2 alert"),
            Self::AUDIT_ALERT => String::from("audit alert"),
            Self::CI_PIPELINES_ALERT => String::from("ci-pipelines alert"),
            Self::CI_TESTS_ALERT => String::from("ci-tests alert"),
            Self::ERROR_TRACKING_ALERT => String::from("error-tracking alert"),
            Self::DATABASE_MONITORING_ALERT => String::from("database-monitoring alert"),
        }
    }
}
