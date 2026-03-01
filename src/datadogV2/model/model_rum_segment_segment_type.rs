// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RumSegmentSegmentType {
    STATIC,
    EVENT_PLATFORM,
    COMBINATION,
    JOURNEYS,
    REFERENCE_TABLE,
    TEMPLATES,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RumSegmentSegmentType {
    fn to_string(&self) -> String {
        match self {
            Self::STATIC => String::from("static"),
            Self::EVENT_PLATFORM => String::from("event_platform"),
            Self::COMBINATION => String::from("combination"),
            Self::JOURNEYS => String::from("journeys"),
            Self::REFERENCE_TABLE => String::from("reference_table"),
            Self::TEMPLATES => String::from("templates"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RumSegmentSegmentType {
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

impl<'de> Deserialize<'de> for RumSegmentSegmentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "static" => Self::STATIC,
            "event_platform" => Self::EVENT_PLATFORM,
            "combination" => Self::COMBINATION,
            "journeys" => Self::JOURNEYS,
            "reference_table" => Self::REFERENCE_TABLE,
            "templates" => Self::TEMPLATES,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
