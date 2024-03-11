// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntheticsTestProcessStatus {
    NOT_SCHEDULED,
    SCHEDULED,
    FINISHED,
    FINISHED_WITH_ERROR,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SyntheticsTestProcessStatus {
    fn to_string(&self) -> String {
        match self {
            Self::NOT_SCHEDULED => String::from("not_scheduled"),
            Self::SCHEDULED => String::from("scheduled"),
            Self::FINISHED => String::from("finished"),
            Self::FINISHED_WITH_ERROR => String::from("finished_with_error"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SyntheticsTestProcessStatus {
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

impl<'de> Deserialize<'de> for SyntheticsTestProcessStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "not_scheduled" => Self::NOT_SCHEDULED,
            "scheduled" => Self::SCHEDULED,
            "finished" => Self::FINISHED,
            "finished_with_error" => Self::FINISHED_WITH_ERROR,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
