// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SensitiveDataScannerTextReplacementType {
    NONE,
    HASH,
    REPLACEMENT_STRING,
    PARTIAL_REPLACEMENT_FROM_BEGINNING,
    PARTIAL_REPLACEMENT_FROM_END,
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl ToString for SensitiveDataScannerTextReplacementType {
    fn to_string(&self) -> String {
        match self {
            Self::NONE => String::from("none"),
            Self::HASH => String::from("hash"),
            Self::REPLACEMENT_STRING => String::from("replacement_string"),
            Self::PARTIAL_REPLACEMENT_FROM_BEGINNING => {
                String::from("partial_replacement_from_beginning")
            }
            Self::PARTIAL_REPLACEMENT_FROM_END => String::from("partial_replacement_from_end"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SensitiveDataScannerTextReplacementType {
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

impl<'de> Deserialize<'de> for SensitiveDataScannerTextReplacementType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "none" => Self::NONE,
            "hash" => Self::HASH,
            "replacement_string" => Self::REPLACEMENT_STRING,
            "partial_replacement_from_beginning" => Self::PARTIAL_REPLACEMENT_FROM_BEGINNING,
            "partial_replacement_from_end" => Self::PARTIAL_REPLACEMENT_FROM_END,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObejct {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
