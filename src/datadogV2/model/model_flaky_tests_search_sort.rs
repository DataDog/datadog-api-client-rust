// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlakyTestsSearchSort {
    FQN_ASCENDING,
    FQN_DESCENDING,
    FIRST_FLAKED_ASCENDING,
    FIRST_FLAKED_DESCENDING,
    LAST_FLAKED_ASCENDING,
    LAST_FLAKED_DESCENDING,
    FAILURE_RATE_ASCENDING,
    FAILURE_RATE_DESCENDING,
    PIPELINES_FAILED_ASCENDING,
    PIPELINES_FAILED_DESCENDING,
    PIPELINES_DURATION_LOST_ASCENDING,
    PIPELINES_DURATION_LOST_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for FlakyTestsSearchSort {
    fn to_string(&self) -> String {
        match self {
            Self::FQN_ASCENDING => String::from("fqn"),
            Self::FQN_DESCENDING => String::from("-fqn"),
            Self::FIRST_FLAKED_ASCENDING => String::from("first_flaked"),
            Self::FIRST_FLAKED_DESCENDING => String::from("-first_flaked"),
            Self::LAST_FLAKED_ASCENDING => String::from("last_flaked"),
            Self::LAST_FLAKED_DESCENDING => String::from("-last_flaked"),
            Self::FAILURE_RATE_ASCENDING => String::from("failure_rate"),
            Self::FAILURE_RATE_DESCENDING => String::from("-failure_rate"),
            Self::PIPELINES_FAILED_ASCENDING => String::from("pipelines_failed"),
            Self::PIPELINES_FAILED_DESCENDING => String::from("-pipelines_failed"),
            Self::PIPELINES_DURATION_LOST_ASCENDING => String::from("pipelines_duration_lost"),
            Self::PIPELINES_DURATION_LOST_DESCENDING => String::from("-pipelines_duration_lost"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for FlakyTestsSearchSort {
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

impl<'de> Deserialize<'de> for FlakyTestsSearchSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "fqn" => Self::FQN_ASCENDING,
            "-fqn" => Self::FQN_DESCENDING,
            "first_flaked" => Self::FIRST_FLAKED_ASCENDING,
            "-first_flaked" => Self::FIRST_FLAKED_DESCENDING,
            "last_flaked" => Self::LAST_FLAKED_ASCENDING,
            "-last_flaked" => Self::LAST_FLAKED_DESCENDING,
            "failure_rate" => Self::FAILURE_RATE_ASCENDING,
            "-failure_rate" => Self::FAILURE_RATE_DESCENDING,
            "pipelines_failed" => Self::PIPELINES_FAILED_ASCENDING,
            "-pipelines_failed" => Self::PIPELINES_FAILED_DESCENDING,
            "pipelines_duration_lost" => Self::PIPELINES_DURATION_LOST_ASCENDING,
            "-pipelines_duration_lost" => Self::PIPELINES_DURATION_LOST_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
