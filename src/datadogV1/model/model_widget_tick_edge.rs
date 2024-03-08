// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetTickEdge {
    BOTTOM,
    LEFT,
    RIGHT,
    TOP,
}

impl ToString for WidgetTickEdge {
    fn to_string(&self) -> String {
        match self {
            Self::BOTTOM => String::from("bottom"),
            Self::LEFT => String::from("left"),
            Self::RIGHT => String::from("right"),
            Self::TOP => String::from("top"),
        }
    }
}

impl Serialize for WidgetTickEdge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetTickEdge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "bottom" => Self::BOTTOM,
            "left" => Self::LEFT,
            "right" => Self::RIGHT,
            "top" => Self::TOP,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
