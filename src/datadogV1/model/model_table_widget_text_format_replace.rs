// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Replace rule for the table widget text format.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TableWidgetTextFormatReplace {
    TableWidgetTextFormatReplaceAll(Box<crate::datadogV1::model::TableWidgetTextFormatReplaceAll>),
    TableWidgetTextFormatReplaceSubstring(
        Box<crate::datadogV1::model::TableWidgetTextFormatReplaceSubstring>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TableWidgetTextFormatReplace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TableWidgetTextFormatReplaceAll>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TableWidgetTextFormatReplace::TableWidgetTextFormatReplaceAll(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::TableWidgetTextFormatReplaceSubstring>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TableWidgetTextFormatReplace::TableWidgetTextFormatReplaceSubstring(_v));
            }
        }

        return Ok(TableWidgetTextFormatReplace::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
