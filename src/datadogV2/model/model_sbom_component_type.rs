// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SBOMComponentType {
    APPLICATION,
    CONTAINER,
    DATA,
    DEVICE,
    DEVICE_DRIVER,
    FILE,
    FIRMWARE,
    FRAMEWORK,
    LIBRARY,
    MACHINE_LEARNING_MODEL,
    OPERATING_SYSTEM,
    PLATFORM,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SBOMComponentType {
    fn to_string(&self) -> String {
        match self {
            Self::APPLICATION => String::from("application"),
            Self::CONTAINER => String::from("container"),
            Self::DATA => String::from("data"),
            Self::DEVICE => String::from("device"),
            Self::DEVICE_DRIVER => String::from("device-driver"),
            Self::FILE => String::from("file"),
            Self::FIRMWARE => String::from("firmware"),
            Self::FRAMEWORK => String::from("framework"),
            Self::LIBRARY => String::from("library"),
            Self::MACHINE_LEARNING_MODEL => String::from("machine-learning-model"),
            Self::OPERATING_SYSTEM => String::from("operating-system"),
            Self::PLATFORM => String::from("platform"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SBOMComponentType {
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

impl<'de> Deserialize<'de> for SBOMComponentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "application" => Self::APPLICATION,
            "container" => Self::CONTAINER,
            "data" => Self::DATA,
            "device" => Self::DEVICE,
            "device-driver" => Self::DEVICE_DRIVER,
            "file" => Self::FILE,
            "firmware" => Self::FIRMWARE,
            "framework" => Self::FRAMEWORK,
            "library" => Self::LIBRARY,
            "machine-learning-model" => Self::MACHINE_LEARNING_MODEL,
            "operating-system" => Self::OPERATING_SYSTEM,
            "platform" => Self::PLATFORM,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
