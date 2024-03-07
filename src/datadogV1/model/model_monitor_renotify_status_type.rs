// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MonitorRenotifyStatusType {
    #[serde(rename = "alert")]
    ALERT,
    #[serde(rename = "warn")]
    WARN,
    #[serde(rename = "no data")]
    NO_DATA,
}
impl ToString for MonitorRenotifyStatusType {
    fn to_string(&self) -> String {
        match self {
            Self::ALERT => String::from("alert"),
            Self::WARN => String::from("warn"),
            Self::NO_DATA => String::from("no data"),
        }
    }
}
