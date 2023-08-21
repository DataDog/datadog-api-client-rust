// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CheckStatusWidgetDefinitionType {
    #[serde(rename = "check_status")]
	CHECK_STATUS,
}

impl ToString for CheckStatusWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::CHECK_STATUS => String::from("check_status"),
        }
    }
}

impl Default for CheckStatusWidgetDefinitionType {
    fn default() -> CheckStatusWidgetDefinitionType {
        Self::CHECK_STATUS
    }
}
