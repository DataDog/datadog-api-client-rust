// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DashboardInlineSchema49 {
	CUSTOM_BG,
	CUSTOM_TEXT,
	CUSTOM_IMAGE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for DashboardInlineSchema49 {
    fn to_string(&self) -> String {
        match self {
            Self::CUSTOM_BG => String::from("custom_bg"),
            Self::CUSTOM_TEXT => String::from("custom_text"),
            Self::CUSTOM_IMAGE => String::from("custom_image"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for DashboardInlineSchema49 {
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

impl<'de> Deserialize<'de> for DashboardInlineSchema49 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"custom_bg" => Self::CUSTOM_BG,"custom_text" => Self::CUSTOM_TEXT,"custom_image" => Self::CUSTOM_IMAGE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
