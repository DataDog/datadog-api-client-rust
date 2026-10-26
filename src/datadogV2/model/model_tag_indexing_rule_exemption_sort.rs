// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TagIndexingRuleExemptionSort {
    NAME,
    NAME_DESCENDING,
    ESTIMATED_CHANGE_IN_VOLUME,
    ESTIMATED_CHANGE_IN_VOLUME_DESCENDING,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for TagIndexingRuleExemptionSort {
    fn to_string(&self) -> String {
        match self {
            Self::NAME => String::from("name"),
            Self::NAME_DESCENDING => String::from("-name"),
            Self::ESTIMATED_CHANGE_IN_VOLUME => String::from("estimated_change_in_volume"),
            Self::ESTIMATED_CHANGE_IN_VOLUME_DESCENDING => {
                String::from("-estimated_change_in_volume")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for TagIndexingRuleExemptionSort {
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

impl<'de> Deserialize<'de> for TagIndexingRuleExemptionSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "name" => Self::NAME,
            "-name" => Self::NAME_DESCENDING,
            "estimated_change_in_volume" => Self::ESTIMATED_CHANGE_IN_VOLUME,
            "-estimated_change_in_volume" => Self::ESTIMATED_CHANGE_IN_VOLUME_DESCENDING,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
