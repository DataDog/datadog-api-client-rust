// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceDefinitionV2Version {
    #[serde(rename = "v2")]
	V2,
}

impl ToString for ServiceDefinitionV2Version {
    fn to_string(&self) -> String {
        match self {
            Self::V2 => String::from("v2"),
        }
    }
}

impl Default for ServiceDefinitionV2Version {
    fn default() -> ServiceDefinitionV2Version {
        Self::V2
    }
}
