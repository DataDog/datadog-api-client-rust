// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonitorSummaryWidgetDefinitionType {
    #[serde(rename = "manage_status")]
    MANAGE_STATUS,
}

impl ToString for MonitorSummaryWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::MANAGE_STATUS => String::from("manage_status"),
        }
    }
}
