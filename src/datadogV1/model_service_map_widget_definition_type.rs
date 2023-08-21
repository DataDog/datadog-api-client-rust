// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceMapWidgetDefinitionType {
    #[serde(rename = "servicemap")]
	SERVICEMAP,
}

impl ToString for ServiceMapWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::SERVICEMAP => String::from("servicemap"),
        }
    }
}

impl Default for ServiceMapWidgetDefinitionType {
    fn default() -> ServiceMapWidgetDefinitionType {
        Self::SERVICEMAP
    }
}
