// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityMonitoringSKU {
    PER_GB_ANALYZED,
    PER_EVENT_IN_SIEM_INDEX_2023,
    ADD_ON_2024,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SecurityMonitoringSKU {
    fn to_string(&self) -> String {
        match self {
            Self::PER_GB_ANALYZED => String::from("per_gb_analyzed"),
            Self::PER_EVENT_IN_SIEM_INDEX_2023 => String::from("per_event_in_siem_index_2023"),
            Self::ADD_ON_2024 => String::from("add_on_2024"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SecurityMonitoringSKU {
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

impl<'de> Deserialize<'de> for SecurityMonitoringSKU {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "per_gb_analyzed" => Self::PER_GB_ANALYZED,
            "per_event_in_siem_index_2023" => Self::PER_EVENT_IN_SIEM_INDEX_2023,
            "add_on_2024" => Self::ADD_ON_2024,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
