// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Used for calendar-aligned time spans, such as the current month or previous year.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetCalendarAlignedSpan {
    /// Whether to hide incomplete cost data in the widget.
    #[serde(rename = "hide_incomplete_cost_data")]
    pub hide_incomplete_cost_data: Option<bool>,
    /// Number of completed periods before the current period. 0 represents the current period.
    #[serde(rename = "offset")]
    pub offset: i64,
    /// Time zone used to align the calendar period.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// Calendar-aligned time span type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::WidgetCalendarAlignedSpanType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetCalendarAlignedSpan {
    pub fn new(
        offset: i64,
        type_: crate::datadogV1::model::WidgetCalendarAlignedSpanType,
    ) -> WidgetCalendarAlignedSpan {
        WidgetCalendarAlignedSpan {
            hide_incomplete_cost_data: None,
            offset,
            timezone: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn hide_incomplete_cost_data(mut self, value: bool) -> Self {
        self.hide_incomplete_cost_data = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for WidgetCalendarAlignedSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetCalendarAlignedSpanVisitor;
        impl<'a> Visitor<'a> for WidgetCalendarAlignedSpanVisitor {
            type Value = WidgetCalendarAlignedSpan;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hide_incomplete_cost_data: Option<bool> = None;
                let mut offset: Option<i64> = None;
                let mut timezone: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::WidgetCalendarAlignedSpanType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hide_incomplete_cost_data" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_incomplete_cost_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "offset" => {
                            offset = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::WidgetCalendarAlignedSpanType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let offset = offset.ok_or_else(|| M::Error::missing_field("offset"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = WidgetCalendarAlignedSpan {
                    hide_incomplete_cost_data,
                    offset,
                    timezone,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetCalendarAlignedSpanVisitor)
    }
}
