// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudConfigurationRuleType {
    #[serde(rename = "cloud_configuration")]
	CLOUD_CONFIGURATION,
}

impl ToString for CloudConfigurationRuleType {
    fn to_string(&self) -> String {
        match self {
            Self::CLOUD_CONFIGURATION => String::from("cloud_configuration"),
        }
    }
}

impl Default for CloudConfigurationRuleType {
    fn default() -> CloudConfigurationRuleType {
        Self::CLOUD_CONFIGURATION
    }
}
