// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration of inline or automatic legends.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SunburstWidgetLegendInlineAutomatic {
    /// Whether to hide the percentages of the groups.
    #[serde(rename = "hide_percent")]
    pub hide_percent: Option<bool>,
    /// Whether to hide the values of the groups.
    #[serde(rename = "hide_value")]
    pub hide_value: Option<bool>,
    /// Whether to show the legend inline or let it be automatically generated.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SunburstWidgetLegendInlineAutomaticType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SunburstWidgetLegendInlineAutomatic {
    pub fn new(
        type_: crate::datadogV1::model::SunburstWidgetLegendInlineAutomaticType,
    ) -> SunburstWidgetLegendInlineAutomatic {
        SunburstWidgetLegendInlineAutomatic {
            hide_percent: None,
            hide_value: None,
            type_,
            _unparsed: false,
        }
    }

    pub fn hide_percent(&mut self, value: bool) -> &mut Self {
        self.hide_percent = Some(value);
        self
    }

    pub fn hide_value(&mut self, value: bool) -> &mut Self {
        self.hide_value = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SunburstWidgetLegendInlineAutomatic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SunburstWidgetLegendInlineAutomaticVisitor;
        impl<'a> Visitor<'a> for SunburstWidgetLegendInlineAutomaticVisitor {
            type Value = SunburstWidgetLegendInlineAutomatic;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut hide_percent: Option<bool> = None;
                let mut hide_value: Option<bool> = None;
                let mut type_: Option<
                    crate::datadogV1::model::SunburstWidgetLegendInlineAutomaticType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "hide_percent" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_percent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hide_value" => {
                            if v.is_null() {
                                continue;
                            }
                            hide_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SunburstWidgetLegendInlineAutomaticType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SunburstWidgetLegendInlineAutomatic {
                    hide_percent,
                    hide_value,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SunburstWidgetLegendInlineAutomaticVisitor)
    }
}
