// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MonitorOptionsNotificationPresets {
    #[serde(rename = "show_all")]
    SHOW_ALL,
    #[serde(rename = "hide_query")]
    HIDE_QUERY,
    #[serde(rename = "hide_handles")]
    HIDE_HANDLES,
    #[serde(rename = "hide_all")]
    HIDE_ALL,
}

impl ToString for MonitorOptionsNotificationPresets {
    fn to_string(&self) -> String {
        match self {
            Self::SHOW_ALL => String::from("show_all"),
            Self::HIDE_QUERY => String::from("hide_query"),
            Self::HIDE_HANDLES => String::from("hide_handles"),
            Self::HIDE_ALL => String::from("hide_all"),
        }
    }
}
