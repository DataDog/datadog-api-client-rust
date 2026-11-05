// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiGatewayCaptureCreateRequestDataAttributesMode {
    METADATA_ONLY,
    FULL_CAPTURE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AiGatewayCaptureCreateRequestDataAttributesMode {
    fn to_string(&self) -> String {
        match self {
            Self::METADATA_ONLY => String::from("metadata_only"),
            Self::FULL_CAPTURE => String::from("full_capture"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AiGatewayCaptureCreateRequestDataAttributesMode {
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

impl<'de> Deserialize<'de> for AiGatewayCaptureCreateRequestDataAttributesMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "metadata_only" => Self::METADATA_ONLY,
            "full_capture" => Self::FULL_CAPTURE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
