// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConfluentResourceType {
    #[serde(rename = "confluent-cloud-resources")]
	CONFLUENT_CLOUD_RESOURCES,
}

impl ToString for ConfluentResourceType {
    fn to_string(&self) -> String {
        match self {
            Self::CONFLUENT_CLOUD_RESOURCES => String::from("confluent-cloud-resources"),
        }
    }
}

impl Default for ConfluentResourceType {
    fn default() -> ConfluentResourceType {
        Self::CONFLUENT_CLOUD_RESOURCES
    }
}
