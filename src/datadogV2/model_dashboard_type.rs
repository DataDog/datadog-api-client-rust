// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DashboardType {
    #[serde(rename = "custom_timeboard")]
	CUSTOM_TIMEBOARD,
    #[serde(rename = "custom_screenboard")]
	CUSTOM_SCREENBOARD,
    #[serde(rename = "integration_screenboard")]
	INTEGRATION_SCREENBOARD,
    #[serde(rename = "integration_timeboard")]
	INTEGRATION_TIMEBOARD,
    #[serde(rename = "host_timeboard")]
	HOST_TIMEBOARD,
}

impl ToString for DashboardType {
    fn to_string(&self) -> String {
        match self {
            Self::CUSTOM_TIMEBOARD => String::from("custom_timeboard"),
            Self::CUSTOM_SCREENBOARD => String::from("custom_screenboard"),
            Self::INTEGRATION_SCREENBOARD => String::from("integration_screenboard"),
            Self::INTEGRATION_TIMEBOARD => String::from("integration_timeboard"),
            Self::HOST_TIMEBOARD => String::from("host_timeboard"),
        }
    }
}

impl Default for DashboardType {
    fn default() -> DashboardType {
        Self::CUSTOM_TIMEBOARD
    }
}
