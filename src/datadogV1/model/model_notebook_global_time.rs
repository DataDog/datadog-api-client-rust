// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Notebook global timeframe.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NotebookGlobalTime {
    NotebookRelativeTime(Box<crate::datadogV1::model::NotebookRelativeTime>),
    NotebookAbsoluteTime(Box<crate::datadogV1::model::NotebookAbsoluteTime>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for NotebookGlobalTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::NotebookRelativeTime>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(NotebookGlobalTime::NotebookRelativeTime(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::NotebookAbsoluteTime>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(NotebookGlobalTime::NotebookAbsoluteTime(_v));
            }
        }

        return Ok(NotebookGlobalTime::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
