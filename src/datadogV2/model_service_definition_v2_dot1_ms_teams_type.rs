// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceDefinitionV2Dot1MSTeamsType {
    #[serde(rename = "microsoft-teams")]
	MICROSOFT_TEAMS,
}

impl ToString for ServiceDefinitionV2Dot1MSTeamsType {
    fn to_string(&self) -> String {
        match self {
            Self::MICROSOFT_TEAMS => String::from("microsoft-teams"),
        }
    }
}

impl Default for ServiceDefinitionV2Dot1MSTeamsType {
    fn default() -> ServiceDefinitionV2Dot1MSTeamsType {
        Self::MICROSOFT_TEAMS
    }
}
