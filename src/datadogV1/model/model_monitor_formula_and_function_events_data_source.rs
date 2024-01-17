// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonitorFormulaAndFunctionEventsDataSource {
    #[serde(rename = "rum")]
    RUM,
    #[serde(rename = "ci_pipelines")]
    CI_PIPELINES,
    #[serde(rename = "ci_tests")]
    CI_TESTS,
    #[serde(rename = "audit")]
    AUDIT,
    #[serde(rename = "events")]
    EVENTS,
    #[serde(rename = "logs")]
    LOGS,
    #[serde(rename = "spans")]
    SPANS,
}

impl ToString for MonitorFormulaAndFunctionEventsDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::RUM => String::from("rum"),
            Self::CI_PIPELINES => String::from("ci_pipelines"),
            Self::CI_TESTS => String::from("ci_tests"),
            Self::AUDIT => String::from("audit"),
            Self::EVENTS => String::from("events"),
            Self::LOGS => String::from("logs"),
            Self::SPANS => String::from("spans"),
        }
    }
}
