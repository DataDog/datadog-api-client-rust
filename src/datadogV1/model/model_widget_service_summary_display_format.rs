// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidgetServiceSummaryDisplayFormat {
    ONE_COLUMN,
    TWO_COLUMN,
    THREE_COLUMN,
}

impl ToString for WidgetServiceSummaryDisplayFormat {
    fn to_string(&self) -> String {
        match self {
            Self::ONE_COLUMN => String::from("one_column"),
            Self::TWO_COLUMN => String::from("two_column"),
            Self::THREE_COLUMN => String::from("three_column"),
        }
    }
}

impl Serialize for WidgetServiceSummaryDisplayFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for WidgetServiceSummaryDisplayFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "one_column" => Self::ONE_COLUMN,
            "two_column" => Self::TWO_COLUMN,
            "three_column" => Self::THREE_COLUMN,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid value for SyntheticsDeviceID: {}",
                    s
                )))
            }
        })
    }
}
