// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OnCallPageLinkRelationshipType {
    RELATES_TO,
    CAUSES,
    BLOCKS,
    DUPLICATES,
    PARENT_OF,
    SUCCESSOR_OF,
    ESCALATES_TO,
    PAGES,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OnCallPageLinkRelationshipType {
    fn to_string(&self) -> String {
        match self {
            Self::RELATES_TO => String::from("relates_to"),
            Self::CAUSES => String::from("causes"),
            Self::BLOCKS => String::from("blocks"),
            Self::DUPLICATES => String::from("duplicates"),
            Self::PARENT_OF => String::from("parent_of"),
            Self::SUCCESSOR_OF => String::from("successor_of"),
            Self::ESCALATES_TO => String::from("escalates_to"),
            Self::PAGES => String::from("pages"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OnCallPageLinkRelationshipType {
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

impl<'de> Deserialize<'de> for OnCallPageLinkRelationshipType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "relates_to" => Self::RELATES_TO,
            "causes" => Self::CAUSES,
            "blocks" => Self::BLOCKS,
            "duplicates" => Self::DUPLICATES,
            "parent_of" => Self::PARENT_OF,
            "successor_of" => Self::SUCCESSOR_OF,
            "escalates_to" => Self::ESCALATES_TO,
            "pages" => Self::PAGES,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
