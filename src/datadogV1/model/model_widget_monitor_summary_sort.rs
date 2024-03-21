// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetMonitorSummarySort {
    NAME,
    GROUP,
    STATUS,
    TAGS,
    TRIGGERED,
    GROUP_ASCENDING,
    GROUP_DESCENDING,
    NAME_ASCENDING,
    NAME_DESCENDING,
    STATUS_ASCENDING,
    STATUS_DESCENDING,
    TAGS_ASCENDING,
    TAGS_DESCENDING,
    TRIGGERED_ASCENDING,
    TRIGGERED_DESCENDING,
    PRIORITY_ASCENDING,
    PRIORITY_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for WidgetMonitorSummarySort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::GROUP => String::from("group"),
            Self::STATUS => String::from("status"),
            Self::TAGS => String::from("tags"),
            Self::TRIGGERED => String::from("triggered"),
            Self::GROUP_ASCENDING => String::from("group,asc"),
            Self::GROUP_DESCENDING => String::from("group,desc"),
            Self::NAME_ASCENDING => String::from("name,asc"),
            Self::NAME_DESCENDING => String::from("name,desc"),
            Self::STATUS_ASCENDING => String::from("status,asc"),
            Self::STATUS_DESCENDING => String::from("status,desc"),
            Self::TAGS_ASCENDING => String::from("tags,asc"),
            Self::TAGS_DESCENDING => String::from("tags,desc"),
            Self::TRIGGERED_ASCENDING => String::from("triggered,asc"),
            Self::TRIGGERED_DESCENDING => String::from("triggered,desc"),
            Self::PRIORITY_ASCENDING => String::from("priority,asc"),
            Self::PRIORITY_DESCENDING => String::from("priority,desc"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for WidgetMonitorSummarySort {
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

impl<'de> Deserialize<'de> for WidgetMonitorSummarySort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME,
            "group" => Self::GROUP,
            "status" => Self::STATUS,
            "tags" => Self::TAGS,
            "triggered" => Self::TRIGGERED,
            "group,asc" => Self::GROUP_ASCENDING,
            "group,desc" => Self::GROUP_DESCENDING,
            "name,asc" => Self::NAME_ASCENDING,
            "name,desc" => Self::NAME_DESCENDING,
            "status,asc" => Self::STATUS_ASCENDING,
            "status,desc" => Self::STATUS_DESCENDING,
            "tags,asc" => Self::TAGS_ASCENDING,
            "tags,desc" => Self::TAGS_DESCENDING,
            "triggered,asc" => Self::TRIGGERED_ASCENDING,
            "triggered,desc" => Self::TRIGGERED_DESCENDING,
            "priority,asc" => Self::PRIORITY_ASCENDING,
            "priority,desc" => Self::PRIORITY_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
