// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringBatchMigrationResourceStatus {
    PENDING,
    IN_PROGRESS,
    COMPLETED,
    CANCELED,
    FAILED,
    PUBLISHING,
    PUBLISHED,
    DUPLICATE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringBatchMigrationResourceStatus {
    fn to_string(&self) -> String {
        match self {
            Self::PENDING => String::from("pending"),
            Self::IN_PROGRESS => String::from("in_progress"),
            Self::COMPLETED => String::from("completed"),
            Self::CANCELED => String::from("canceled"),
            Self::FAILED => String::from("failed"),
            Self::PUBLISHING => String::from("publishing"),
            Self::PUBLISHED => String::from("published"),
            Self::DUPLICATE => String::from("duplicate"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringBatchMigrationResourceStatus {
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

impl<'de> Deserialize<'de> for SecurityMonitoringBatchMigrationResourceStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pending" => Self::PENDING,
            "in_progress" => Self::IN_PROGRESS,
            "completed" => Self::COMPLETED,
            "canceled" => Self::CANCELED,
            "failed" => Self::FAILED,
            "publishing" => Self::PUBLISHING,
            "published" => Self::PUBLISHED,
            "duplicate" => Self::DUPLICATE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
