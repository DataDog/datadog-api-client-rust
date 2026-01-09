// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CreateStatusPageRequestDataAttributesVisualizationType {
    BARS_AND_UPTIME_PERCENTAGE,
    BARS_ONLY,
    COMPONENT_NAME_ONLY,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for CreateStatusPageRequestDataAttributesVisualizationType {
    fn to_string(&self) -> String {
        match self {
            Self::BARS_AND_UPTIME_PERCENTAGE => String::from("bars_and_uptime_percentage"),
            Self::BARS_ONLY => String::from("bars_only"),
            Self::COMPONENT_NAME_ONLY => String::from("component_name_only"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for CreateStatusPageRequestDataAttributesVisualizationType {
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

impl<'de> Deserialize<'de> for CreateStatusPageRequestDataAttributesVisualizationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "bars_and_uptime_percentage" => Self::BARS_AND_UPTIME_PERCENTAGE,
            "bars_only" => Self::BARS_ONLY,
            "component_name_only" => Self::COMPONENT_NAME_ONLY,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
