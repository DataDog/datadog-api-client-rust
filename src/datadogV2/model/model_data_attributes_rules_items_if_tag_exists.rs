// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataAttributesRulesItemsIfTagExists {
    APPEND,
    DO_NOT_APPLY,
    REPLACE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for DataAttributesRulesItemsIfTagExists {
    fn to_string(&self) -> String {
        match self {
            Self::APPEND => String::from("append"),
            Self::DO_NOT_APPLY => String::from("do_not_apply"),
            Self::REPLACE => String::from("replace"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for DataAttributesRulesItemsIfTagExists {
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

impl<'de> Deserialize<'de> for DataAttributesRulesItemsIfTagExists {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "append" => Self::APPEND,
            "do_not_apply" => Self::DO_NOT_APPLY,
            "replace" => Self::REPLACE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
