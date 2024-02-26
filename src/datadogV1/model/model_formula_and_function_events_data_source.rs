// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FormulaAndFunctionEventsDataSource {
    #[serde(rename = "logs")]
    LOGS,
    #[serde(rename = "spans")]
    SPANS,
    #[serde(rename = "network")]
    NETWORK,
    #[serde(rename = "rum")]
    RUM,
    #[serde(rename = "security_signals")]
    SECURITY_SIGNALS,
    #[serde(rename = "profiles")]
    PROFILES,
    #[serde(rename = "audit")]
    AUDIT,
    #[serde(rename = "events")]
    EVENTS,
    #[serde(rename = "ci_tests")]
    CI_TESTS,
    #[serde(rename = "ci_pipelines")]
    CI_PIPELINES,
}

impl ToString for FormulaAndFunctionEventsDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS => String::from("logs"),
            Self::SPANS => String::from("spans"),
            Self::NETWORK => String::from("network"),
            Self::RUM => String::from("rum"),
            Self::SECURITY_SIGNALS => String::from("security_signals"),
            Self::PROFILES => String::from("profiles"),
            Self::AUDIT => String::from("audit"),
            Self::EVENTS => String::from("events"),
            Self::CI_TESTS => String::from("ci_tests"),
            Self::CI_PIPELINES => String::from("ci_pipelines"),
        }
    }
}
