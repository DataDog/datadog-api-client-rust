// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScatterplotWidgetAggregator {
    AVERAGE,
    LAST,
    MAXIMUM,
    MINIMUM,
    SUM,
}

impl ToString for ScatterplotWidgetAggregator {
    fn to_string(&self) -> String {
        match self {
            Self::AVERAGE => String::from("avg"),
            Self::LAST => String::from("last"),
            Self::MAXIMUM => String::from("max"),
            Self::MINIMUM => String::from("min"),
            Self::SUM => String::from("sum"),
        }
    }
}

impl Serialize for ScatterplotWidgetAggregator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ScatterplotWidgetAggregator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "avg" => Self::AVERAGE,
            "last" => Self::LAST,
            "max" => Self::MAXIMUM,
            "min" => Self::MINIMUM,
            "sum" => Self::SUM,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
