// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageSizing {
	NONE,
	CENTER,
	FILL,
	CONTAIN,
	COVER,
	SCALE_DOWN,
	ZOOM,
	FIT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ImageSizing {
    fn to_string(&self) -> String {
        match self {
            Self::NONE => String::from("none"),
            Self::CENTER => String::from("center"),
            Self::FILL => String::from("fill"),
            Self::CONTAIN => String::from("contain"),
            Self::COVER => String::from("cover"),
            Self::SCALE_DOWN => String::from("scale-down"),
            Self::ZOOM => String::from("zoom"),
            Self::FIT => String::from("fit"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ImageSizing {
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

impl<'de> Deserialize<'de> for ImageSizing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"none" => Self::NONE,"center" => Self::CENTER,"fill" => Self::FILL,"contain" => Self::CONTAIN,"cover" => Self::COVER,"scale-down" => Self::SCALE_DOWN,"zoom" => Self::ZOOM,"fit" => Self::FIT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}
