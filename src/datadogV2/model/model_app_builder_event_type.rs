// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppBuilderEventType {
    CUSTOM,
    SETCOMPONENTSTATE,
    TRIGGERQUERY,
    OPENMODAL,
    CLOSEMODAL,
    OPENURL,
    DOWNLOADFILE,
    SETSTATEVARIABLEVALUE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AppBuilderEventType {
    fn to_string(&self) -> String {
        match self {
            Self::CUSTOM => String::from("custom"),
            Self::SETCOMPONENTSTATE => String::from("setComponentState"),
            Self::TRIGGERQUERY => String::from("triggerQuery"),
            Self::OPENMODAL => String::from("openModal"),
            Self::CLOSEMODAL => String::from("closeModal"),
            Self::OPENURL => String::from("openUrl"),
            Self::DOWNLOADFILE => String::from("downloadFile"),
            Self::SETSTATEVARIABLEVALUE => String::from("setStateVariableValue"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AppBuilderEventType {
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

impl<'de> Deserialize<'de> for AppBuilderEventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom" => Self::CUSTOM,
            "setComponentState" => Self::SETCOMPONENTSTATE,
            "triggerQuery" => Self::TRIGGERQUERY,
            "openModal" => Self::OPENMODAL,
            "closeModal" => Self::CLOSEMODAL,
            "openUrl" => Self::OPENURL,
            "downloadFile" => Self::DOWNLOADFILE,
            "setStateVariableValue" => Self::SETSTATEVARIABLEVALUE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
