// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IoCScoreEffect {
    RAISE_SCORE,
    LOWER_SCORE,
    NO_EFFECT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IoCScoreEffect {
    fn to_string(&self) -> String {
        match self {
            Self::RAISE_SCORE => String::from("RAISE_SCORE"),
            Self::LOWER_SCORE => String::from("LOWER_SCORE"),
            Self::NO_EFFECT => String::from("NO_EFFECT"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IoCScoreEffect {
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

impl<'de> Deserialize<'de> for IoCScoreEffect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "RAISE_SCORE" => Self::RAISE_SCORE,
            "LOWER_SCORE" => Self::LOWER_SCORE,
            "NO_EFFECT" => Self::NO_EFFECT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
