// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentType {
    TABLE,
    TEXTINPUT,
    TEXTAREA,
    BUTTON,
    TEXT,
    SELECT,
    MODAL,
    SCHEMAFORM,
    CHECKBOX,
    TABS,
    VEGACHART,
    RADIOBUTTONS,
    NUMBERINPUT,
    FILEINPUT,
    JSONINPUT,
    GRIDCELL,
    DATERANGEPICKER,
    SEARCH,
    CONTAINER,
    CALLOUTVALUE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for ComponentType {
    fn to_string(&self) -> String {
        match self {
            Self::TABLE => String::from("table"),
            Self::TEXTINPUT => String::from("textInput"),
            Self::TEXTAREA => String::from("textArea"),
            Self::BUTTON => String::from("button"),
            Self::TEXT => String::from("text"),
            Self::SELECT => String::from("select"),
            Self::MODAL => String::from("modal"),
            Self::SCHEMAFORM => String::from("schemaForm"),
            Self::CHECKBOX => String::from("checkbox"),
            Self::TABS => String::from("tabs"),
            Self::VEGACHART => String::from("vegaChart"),
            Self::RADIOBUTTONS => String::from("radioButtons"),
            Self::NUMBERINPUT => String::from("numberInput"),
            Self::FILEINPUT => String::from("fileInput"),
            Self::JSONINPUT => String::from("jsonInput"),
            Self::GRIDCELL => String::from("gridCell"),
            Self::DATERANGEPICKER => String::from("dateRangePicker"),
            Self::SEARCH => String::from("search"),
            Self::CONTAINER => String::from("container"),
            Self::CALLOUTVALUE => String::from("calloutValue"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for ComponentType {
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

impl<'de> Deserialize<'de> for ComponentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "table" => Self::TABLE,
            "textInput" => Self::TEXTINPUT,
            "textArea" => Self::TEXTAREA,
            "button" => Self::BUTTON,
            "text" => Self::TEXT,
            "select" => Self::SELECT,
            "modal" => Self::MODAL,
            "schemaForm" => Self::SCHEMAFORM,
            "checkbox" => Self::CHECKBOX,
            "tabs" => Self::TABS,
            "vegaChart" => Self::VEGACHART,
            "radioButtons" => Self::RADIOBUTTONS,
            "numberInput" => Self::NUMBERINPUT,
            "fileInput" => Self::FILEINPUT,
            "jsonInput" => Self::JSONINPUT,
            "gridCell" => Self::GRIDCELL,
            "dateRangePicker" => Self::DATERANGEPICKER,
            "search" => Self::SEARCH,
            "container" => Self::CONTAINER,
            "calloutValue" => Self::CALLOUTVALUE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
