// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FunnelSource {
    #[serde(rename = "rum")]
	RUM,
}

impl ToString for FunnelSource {
    fn to_string(&self) -> String {
        match self {
            Self::RUM => String::from("rum"),
        }
    }
}

impl Default for FunnelSource {
    fn default() -> FunnelSource {
        Self::RUM
    }
}
