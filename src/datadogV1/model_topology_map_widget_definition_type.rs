// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TopologyMapWidgetDefinitionType {
    #[serde(rename = "topology_map")]
	TOPOLOGY_MAP,
}

impl ToString for TopologyMapWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::TOPOLOGY_MAP => String::from("topology_map"),
        }
    }
}

impl Default for TopologyMapWidgetDefinitionType {
    fn default() -> TopologyMapWidgetDefinitionType {
        Self::TOPOLOGY_MAP
    }
}
