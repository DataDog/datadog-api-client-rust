// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MonitorType {
    COMPOSITE,
    EVENT_ALERT,
    LOG_ALERT,
    METRIC_ALERT,
    PROCESS_ALERT,
    QUERY_ALERT,
    RUM_ALERT,
    SERVICE_CHECK,
    SYNTHETICS_ALERT,
    TRACE_ANALYTICS_ALERT,
    SLO_ALERT,
    EVENT_V2_ALERT,
    AUDIT_ALERT,
    CI_PIPELINES_ALERT,
    CI_TESTS_ALERT,
    ERROR_TRACKING_ALERT,
    DATABASE_MONITORING_ALERT,
    UnparsedObject(crate::datadog::UnparsedObejct),
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
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for MonitorType {
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

impl<'de> Deserialize<'de> for MonitorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "composite" => Self::COMPOSITE,
            "event alert" => Self::EVENT_ALERT,
            "log alert" => Self::LOG_ALERT,
            "metric alert" => Self::METRIC_ALERT,
            "process alert" => Self::PROCESS_ALERT,
            "query alert" => Self::QUERY_ALERT,
            "rum alert" => Self::RUM_ALERT,
            "service check" => Self::SERVICE_CHECK,
            "synthetics alert" => Self::SYNTHETICS_ALERT,
            "trace-analytics alert" => Self::TRACE_ANALYTICS_ALERT,
            "slo alert" => Self::SLO_ALERT,
            "event-v2 alert" => Self::EVENT_V2_ALERT,
            "audit alert" => Self::AUDIT_ALERT,
            "ci-pipelines alert" => Self::CI_PIPELINES_ALERT,
            "ci-tests alert" => Self::CI_TESTS_ALERT,
            "error-tracking alert" => Self::ERROR_TRACKING_ALERT,
            "database-monitoring alert" => Self::DATABASE_MONITORING_ALERT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
