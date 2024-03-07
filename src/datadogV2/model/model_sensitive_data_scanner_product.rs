// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SensitiveDataScannerProduct {
    #[serde(rename = "logs")]
    LOGS,
    #[serde(rename = "rum")]
    RUM,
    #[serde(rename = "events")]
    EVENTS,
    #[serde(rename = "apm")]
    APM,
}

impl ToString for SensitiveDataScannerProduct {
    fn to_string(&self) -> String {
        match self {
            Self::LOGS => String::from("logs"),
            Self::RUM => String::from("rum"),
            Self::EVENTS => String::from("events"),
            Self::APM => String::from("apm"),
        }
    }
}
