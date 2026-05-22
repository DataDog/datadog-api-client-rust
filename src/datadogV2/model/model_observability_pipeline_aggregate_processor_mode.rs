// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityPipelineAggregateProcessorMode {
    AUTO,
    SUM,
    LATEST,
    COUNT,
    MAX,
    MIN,
    MEAN,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ObservabilityPipelineAggregateProcessorMode {
    fn to_string(&self) -> String {
        match self {
            Self::AUTO => String::from("auto"),
            Self::SUM => String::from("sum"),
            Self::LATEST => String::from("latest"),
            Self::COUNT => String::from("count"),
            Self::MAX => String::from("max"),
            Self::MIN => String::from("min"),
            Self::MEAN => String::from("mean"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ObservabilityPipelineAggregateProcessorMode {
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

impl<'de> Deserialize<'de> for ObservabilityPipelineAggregateProcessorMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "auto" => Self::AUTO,
            "sum" => Self::SUM,
            "latest" => Self::LATEST,
            "count" => Self::COUNT,
            "max" => Self::MAX,
            "min" => Self::MIN,
            "mean" => Self::MEAN,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
