// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DistributionPointsContentEncoding {
    #[serde(rename = "deflate")]
	DEFLATE,
}

impl ToString for DistributionPointsContentEncoding {
    fn to_string(&self) -> String {
        match self {
            Self::DEFLATE => String::from("deflate"),
        }
    }
}

impl Default for DistributionPointsContentEncoding {
    fn default() -> DistributionPointsContentEncoding {
        Self::DEFLATE
    }
}
