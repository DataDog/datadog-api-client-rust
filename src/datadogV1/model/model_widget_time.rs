// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Time setting for the widget.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WidgetTime {
    WidgetNewLiveSpan(Box<crate::datadogV1::model::WidgetNewLiveSpan>),
    WidgetNewFixedSpan(Box<crate::datadogV1::model::WidgetNewFixedSpan>),
    WidgetLegacyLiveSpan(Box<crate::datadogV1::model::WidgetLegacyLiveSpan>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for WidgetTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::WidgetNewLiveSpan>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetTime::WidgetNewLiveSpan(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::WidgetNewFixedSpan>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetTime::WidgetNewFixedSpan(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::WidgetLegacyLiveSpan>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(WidgetTime::WidgetLegacyLiveSpan(_v));
            }
        }

        return Ok(WidgetTime::UnparsedObject(crate::datadog::UnparsedObject {
            value,
        }));
    }
}
