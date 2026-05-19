// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaseInsightType {
    SECURITY_SIGNAL,
    MONITOR,
    EVENT_CORRELATION,
    ERROR_TRACKING,
    CLOUD_COST_RECOMMENDATION,
    INCIDENT,
    SENSITIVE_DATA_SCANNER_ISSUE,
    EVENT,
    WATCHDOG_STORY,
    WIDGET,
    SECURITY_FINDING,
    INSIGHT_SCORECARD_CAMPAIGN,
    RESOURCE_POLICY,
    APM_RECOMMENDATION,
    SCM_URL,
    PROFILING_DOWNSIZING_EXPERIMENT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CaseInsightType {
    fn to_string(&self) -> String {
        match self {
            Self::SECURITY_SIGNAL => String::from("SECURITY_SIGNAL"),
            Self::MONITOR => String::from("MONITOR"),
            Self::EVENT_CORRELATION => String::from("EVENT_CORRELATION"),
            Self::ERROR_TRACKING => String::from("ERROR_TRACKING"),
            Self::CLOUD_COST_RECOMMENDATION => String::from("CLOUD_COST_RECOMMENDATION"),
            Self::INCIDENT => String::from("INCIDENT"),
            Self::SENSITIVE_DATA_SCANNER_ISSUE => String::from("SENSITIVE_DATA_SCANNER_ISSUE"),
            Self::EVENT => String::from("EVENT"),
            Self::WATCHDOG_STORY => String::from("WATCHDOG_STORY"),
            Self::WIDGET => String::from("WIDGET"),
            Self::SECURITY_FINDING => String::from("SECURITY_FINDING"),
            Self::INSIGHT_SCORECARD_CAMPAIGN => String::from("INSIGHT_SCORECARD_CAMPAIGN"),
            Self::RESOURCE_POLICY => String::from("RESOURCE_POLICY"),
            Self::APM_RECOMMENDATION => String::from("APM_RECOMMENDATION"),
            Self::SCM_URL => String::from("SCM_URL"),
            Self::PROFILING_DOWNSIZING_EXPERIMENT => {
                String::from("PROFILING_DOWNSIZING_EXPERIMENT")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CaseInsightType {
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

impl<'de> Deserialize<'de> for CaseInsightType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "SECURITY_SIGNAL" => Self::SECURITY_SIGNAL,
            "MONITOR" => Self::MONITOR,
            "EVENT_CORRELATION" => Self::EVENT_CORRELATION,
            "ERROR_TRACKING" => Self::ERROR_TRACKING,
            "CLOUD_COST_RECOMMENDATION" => Self::CLOUD_COST_RECOMMENDATION,
            "INCIDENT" => Self::INCIDENT,
            "SENSITIVE_DATA_SCANNER_ISSUE" => Self::SENSITIVE_DATA_SCANNER_ISSUE,
            "EVENT" => Self::EVENT,
            "WATCHDOG_STORY" => Self::WATCHDOG_STORY,
            "WIDGET" => Self::WIDGET,
            "SECURITY_FINDING" => Self::SECURITY_FINDING,
            "INSIGHT_SCORECARD_CAMPAIGN" => Self::INSIGHT_SCORECARD_CAMPAIGN,
            "RESOURCE_POLICY" => Self::RESOURCE_POLICY,
            "APM_RECOMMENDATION" => Self::APM_RECOMMENDATION,
            "SCM_URL" => Self::SCM_URL,
            "PROFILING_DOWNSIZING_EXPERIMENT" => Self::PROFILING_DOWNSIZING_EXPERIMENT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
