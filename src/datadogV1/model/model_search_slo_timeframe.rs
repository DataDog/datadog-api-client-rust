// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SearchSLOTimeframe {
    #[serde(rename = "7d")]
    SEVEN_DAYS,
    #[serde(rename = "30d")]
    THIRTY_DAYS,
    #[serde(rename = "90d")]
    NINETY_DAYS,
}
impl ToString for SearchSLOTimeframe {
    fn to_string(&self) -> String {
        match self {
            Self::SEVEN_DAYS => String::from("7d"),
            Self::THIRTY_DAYS => String::from("30d"),
            Self::NINETY_DAYS => String::from("90d"),
        }
    }
}
