// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SLOReportStatus {
    IN_PROGRESS,
    COMPLETED,
    COMPLETED_WITH_ERRORS,
    FAILED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SLOReportStatus {
    fn to_string(&self) -> String {
        match self {
            Self::IN_PROGRESS => String::from("in_progress"),
            Self::COMPLETED => String::from("completed"),
            Self::COMPLETED_WITH_ERRORS => String::from("completed_with_errors"),
            Self::FAILED => String::from("failed"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SLOReportStatus {
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

impl<'de> Deserialize<'de> for SLOReportStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "in_progress" => Self::IN_PROGRESS,
            "completed" => Self::COMPLETED,
            "completed_with_errors" => Self::COMPLETED_WITH_ERRORS,
            "failed" => Self::FAILED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
