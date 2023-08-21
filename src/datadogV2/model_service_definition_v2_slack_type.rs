// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceDefinitionV2SlackType {
    #[serde(rename = "slack")]
	SLACK,
}

impl ToString for ServiceDefinitionV2SlackType {
    fn to_string(&self) -> String {
        match self {
            Self::SLACK => String::from("slack"),
        }
    }
}

impl Default for ServiceDefinitionV2SlackType {
    fn default() -> ServiceDefinitionV2SlackType {
        Self::SLACK
    }
}
