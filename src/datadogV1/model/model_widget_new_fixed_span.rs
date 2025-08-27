// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Used for fixed span times, such as 'March 1 to March 7'.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetNewFixedSpan {
    /// Start time in seconds since epoch.
    #[serde(rename = "from")]
    pub from: i64,
    /// Whether to hide incomplete cost data in the widget.
    #[serde(rename = "hide_incomplete_cost_data")]
    pub hide_incomplete_cost_data: Option<bool>,
    /// End time in seconds since epoch.
    #[serde(rename = "to")]
    pub to: i64,
    /// Type "fixed" denotes a fixed span.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::WidgetNewFixedSpanType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetNewFixedSpan {
    pub fn new(
        from: i64,
        to: i64,
        type_: crate::datadogV1::model::WidgetNewFixedSpanType,
    ) -> WidgetNewFixedSpan {
        WidgetNewFixedSpan {
            from,
            hide_incomplete_cost_data: None,
            to,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn hide_incomplete_cost_data(mut self, value: bool) -> Self {
        self.hide_incomplete_cost_data = Some(value);
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

impl<'de> Deserialize<'de> for WidgetNewFixedSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetNewFixedSpanVisitor;
        impl<'a> Visitor<'a> for WidgetNewFixedSpanVisitor {
            type Value = WidgetNewFixedSpan;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<i64> = None;
                let mut hide_incomplete_cost_data: Option<bool> = None;
                let mut to: Option<i64> = None;
                let mut type_: Option<crate::datadogV1::model::WidgetNewFixedSpanType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hide_incomplete_cost_data" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_incomplete_cost_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::WidgetNewFixedSpanType::UnparsedObject(_type_) => {
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
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = WidgetNewFixedSpan {
                    from,
                    hide_incomplete_cost_data,
                    to,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetNewFixedSpanVisitor)
    }
}
