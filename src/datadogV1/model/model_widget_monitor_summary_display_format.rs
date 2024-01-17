// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetMonitorSummaryDisplayFormat {
    #[serde(rename = "counts")]
    COUNTS,
    #[serde(rename = "countsAndList")]
    COUNTS_AND_LIST,
    #[serde(rename = "list")]
    LIST,
}

impl ToString for WidgetMonitorSummaryDisplayFormat {
    fn to_string(&self) -> String {
        match self {
            Self::COUNTS => String::from("counts"),
            Self::COUNTS_AND_LIST => String::from("countsAndList"),
            Self::LIST => String::from("list"),
        }
    }
}
