// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetImageSizing {
    FILL,
    CONTAIN,
    COVER,
    NONE,
    SCALEDOWN,
    ZOOM,
    FIT,
    CENTER,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for WidgetImageSizing {
    fn to_string(&self) -> String {
        match self {
            Self::FILL => String::from("fill"),
            Self::CONTAIN => String::from("contain"),
            Self::COVER => String::from("cover"),
            Self::NONE => String::from("none"),
            Self::SCALEDOWN => String::from("scale-down"),
            Self::ZOOM => String::from("zoom"),
            Self::FIT => String::from("fit"),
            Self::CENTER => String::from("center"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for WidgetImageSizing {
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

impl<'de> Deserialize<'de> for WidgetImageSizing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "fill" => Self::FILL,
            "contain" => Self::CONTAIN,
            "cover" => Self::COVER,
            "none" => Self::NONE,
            "scale-down" => Self::SCALEDOWN,
            "zoom" => Self::ZOOM,
            "fit" => Self::FIT,
            "center" => Self::CENTER,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
