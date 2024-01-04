// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListStreamSource {
    #[serde(rename = "logs_stream")]
    LOGS_STREAM,
    #[serde(rename = "audit_stream")]
    AUDIT_STREAM,
    #[serde(rename = "ci_pipeline_stream")]
    CI_PIPELINE_STREAM,
    #[serde(rename = "ci_test_stream")]
    CI_TEST_STREAM,
    #[serde(rename = "rum_issue_stream")]
    RUM_ISSUE_STREAM,
    #[serde(rename = "apm_issue_stream")]
    APM_ISSUE_STREAM,
    #[serde(rename = "trace_stream")]
    TRACE_STREAM,
    #[serde(rename = "logs_issue_stream")]
    LOGS_ISSUE_STREAM,
    #[serde(rename = "logs_pattern_stream")]
    LOGS_PATTERN_STREAM,
    #[serde(rename = "logs_transaction_stream")]
    LOGS_TRANSACTION_STREAM,
    #[serde(rename = "event_stream")]
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
