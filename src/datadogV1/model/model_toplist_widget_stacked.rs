// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Top list widget stacked display options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ToplistWidgetStacked {
    /// Top list widget stacked legend behavior.
    #[serde(rename = "legend")]
    pub legend: Option<crate::datadogV1::model::ToplistWidgetLegend>,
    /// Top list widget stacked display type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ToplistWidgetStackedType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ToplistWidgetStacked {
    pub fn new(type_: crate::datadogV1::model::ToplistWidgetStackedType) -> ToplistWidgetStacked {
        ToplistWidgetStacked {
            legend: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn legend(mut self, value: crate::datadogV1::model::ToplistWidgetLegend) -> Self {
        self.legend = Some(value);
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

impl<'de> Deserialize<'de> for ToplistWidgetStacked {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ToplistWidgetStackedVisitor;
        impl<'a> Visitor<'a> for ToplistWidgetStackedVisitor {
            type Value = ToplistWidgetStacked;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut legend: Option<crate::datadogV1::model::ToplistWidgetLegend> = None;
                let mut type_: Option<crate::datadogV1::model::ToplistWidgetStackedType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "legend" => {
                            if v.is_null() {
                                continue;
                            }
                            legend = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _legend) = legend {
                                match _legend {
                                    crate::datadogV1::model::ToplistWidgetLegend::UnparsedObject(_legend) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::ToplistWidgetStackedType::UnparsedObject(_type_) => {
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
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ToplistWidgetStacked {
                    legend,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ToplistWidgetStackedVisitor)
    }
}
