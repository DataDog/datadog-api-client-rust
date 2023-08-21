// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TopologyQueryDataSource {
    #[serde(rename = "data_streams")]
	DATA_STREAMS,
    #[serde(rename = "service_map")]
	SERVICE_MAP,
}

impl ToString for TopologyQueryDataSource {
    fn to_string(&self) -> String {
        match self {
            Self::DATA_STREAMS => String::from("data_streams"),
            Self::SERVICE_MAP => String::from("service_map"),
        }
    }
}

impl Default for TopologyQueryDataSource {
    fn default() -> TopologyQueryDataSource {
        Self::DATA_STREAMS
    }
}
