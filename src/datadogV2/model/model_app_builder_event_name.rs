// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppBuilderEventName {
    PAGECHANGE,
    TABLEROWCLICK,
    TABLEROWBUTTONCLICK,
    CHANGE,
    SUBMIT,
    CLICK,
    TOGGLEOPEN,
    CLOSE,
    OPEN,
    EXECUTIONFINISHED,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for AppBuilderEventName {
    fn to_string(&self) -> String {
        match self {
            Self::PAGECHANGE => String::from("pageChange"),
            Self::TABLEROWCLICK => String::from("tableRowClick"),
            Self::TABLEROWBUTTONCLICK => String::from("_tableRowButtonClick"),
            Self::CHANGE => String::from("change"),
            Self::SUBMIT => String::from("submit"),
            Self::CLICK => String::from("click"),
            Self::TOGGLEOPEN => String::from("toggleOpen"),
            Self::CLOSE => String::from("close"),
            Self::OPEN => String::from("open"),
            Self::EXECUTIONFINISHED => String::from("executionFinished"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for AppBuilderEventName {
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

impl<'de> Deserialize<'de> for AppBuilderEventName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pageChange" => Self::PAGECHANGE,
            "tableRowClick" => Self::TABLEROWCLICK,
            "_tableRowButtonClick" => Self::TABLEROWBUTTONCLICK,
            "change" => Self::CHANGE,
            "submit" => Self::SUBMIT,
            "click" => Self::CLICK,
            "toggleOpen" => Self::TOGGLEOPEN,
            "close" => Self::CLOSE,
            "open" => Self::OPEN,
            "executionFinished" => Self::EXECUTIONFINISHED,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
