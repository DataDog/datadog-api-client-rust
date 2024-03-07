// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DashboardType {
    #[serde(rename = "custom_timeboard")]
    CUSTOM_TIMEBOARD,
    #[serde(rename = "custom_screenboard")]
    CUSTOM_SCREENBOARD,
}

impl ToString for DashboardType {
    fn to_string(&self) -> String {
        match self {
            Self::CUSTOM_TIMEBOARD => String::from("custom_timeboard"),
            Self::CUSTOM_SCREENBOARD => String::from("custom_screenboard"),
        }
    }
}
