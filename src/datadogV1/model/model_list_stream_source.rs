// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListStreamSource {
    LOGS_STREAM,
    AUDIT_STREAM,
    CI_PIPELINE_STREAM,
    CI_TEST_STREAM,
    RUM_ISSUE_STREAM,
    APM_ISSUE_STREAM,
    TRACE_STREAM,
    LOGS_ISSUE_STREAM,
    LOGS_PATTERN_STREAM,
    LOGS_TRANSACTION_STREAM,
    EVENT_STREAM,
}

impl ToString for ListStreamSource {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS_STREAM => String::from("logs_stream"),
            Self::AUDIT_STREAM => String::from("audit_stream"),
            Self::CI_PIPELINE_STREAM => String::from("ci_pipeline_stream"),
            Self::CI_TEST_STREAM => String::from("ci_test_stream"),
            Self::RUM_ISSUE_STREAM => String::from("rum_issue_stream"),
            Self::APM_ISSUE_STREAM => String::from("apm_issue_stream"),
            Self::TRACE_STREAM => String::from("trace_stream"),
            Self::LOGS_ISSUE_STREAM => String::from("logs_issue_stream"),
            Self::LOGS_PATTERN_STREAM => String::from("logs_pattern_stream"),
            Self::LOGS_TRANSACTION_STREAM => String::from("logs_transaction_stream"),
            Self::EVENT_STREAM => String::from("event_stream"),
        }
    }
}

impl Serialize for ListStreamSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ListStreamSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "logs_stream" => Self::LOGS_STREAM,
            "audit_stream" => Self::AUDIT_STREAM,
            "ci_pipeline_stream" => Self::CI_PIPELINE_STREAM,
            "ci_test_stream" => Self::CI_TEST_STREAM,
            "rum_issue_stream" => Self::RUM_ISSUE_STREAM,
            "apm_issue_stream" => Self::APM_ISSUE_STREAM,
            "trace_stream" => Self::TRACE_STREAM,
            "logs_issue_stream" => Self::LOGS_ISSUE_STREAM,
            "logs_pattern_stream" => Self::LOGS_PATTERN_STREAM,
            "logs_transaction_stream" => Self::LOGS_TRANSACTION_STREAM,
            "event_stream" => Self::EVENT_STREAM,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
