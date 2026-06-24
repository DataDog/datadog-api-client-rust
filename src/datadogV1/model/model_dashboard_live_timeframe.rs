// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A live dashboard timeframe.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardLiveTimeframe {
    /// Type of live timeframe.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::DashboardLiveTimeframeType,
    /// Unit of the time span.
    #[serde(rename = "unit")]
    pub unit: crate::datadogV1::model::WidgetLiveSpanUnit,
    /// Value of the live timeframe span.
    #[serde(rename = "value")]
    pub value: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardLiveTimeframe {
    pub fn new(
        type_: crate::datadogV1::model::DashboardLiveTimeframeType,
        unit: crate::datadogV1::model::WidgetLiveSpanUnit,
        value: i64,
    ) -> DashboardLiveTimeframe {
        DashboardLiveTimeframe {
            type_,
            unit,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for DashboardLiveTimeframe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardLiveTimeframeVisitor;
        impl<'a> Visitor<'a> for DashboardLiveTimeframeVisitor {
            type Value = DashboardLiveTimeframe;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut type_: Option<crate::datadogV1::model::DashboardLiveTimeframeType> = None;
                let mut unit: Option<crate::datadogV1::model::WidgetLiveSpanUnit> = None;
                let mut value: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::DashboardLiveTimeframeType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "unit" => {
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unit) = unit {
                                match _unit {
                                    crate::datadogV1::model::WidgetLiveSpanUnit::UnparsedObject(
                                        _unit,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let unit = unit.ok_or_else(|| M::Error::missing_field("unit"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = DashboardLiveTimeframe {
                    type_,
                    unit,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardLiveTimeframeVisitor)
    }
}
