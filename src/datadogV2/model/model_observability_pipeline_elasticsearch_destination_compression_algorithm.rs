// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityPipelineElasticsearchDestinationCompressionAlgorithm {
    NONE,
    GZIP,
    ZLIB,
    ZSTD,
    SNAPPY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ObservabilityPipelineElasticsearchDestinationCompressionAlgorithm {
    fn to_string(&self) -> String {
        match self {
            Self::NONE => String::from("none"),
            Self::GZIP => String::from("gzip"),
            Self::ZLIB => String::from("zlib"),
            Self::ZSTD => String::from("zstd"),
            Self::SNAPPY => String::from("snappy"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ObservabilityPipelineElasticsearchDestinationCompressionAlgorithm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ObservabilityPipelineElasticsearchDestinationCompressionAlgorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "none" => Self::NONE,
            "gzip" => Self::GZIP,
            "zlib" => Self::ZLIB,
            "zstd" => Self::ZSTD,
            "snappy" => Self::SNAPPY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
