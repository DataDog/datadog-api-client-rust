// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number format options for the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WidgetNumberFormat {
    /// Number format unit.
    #[serde(rename = "unit")]
    pub unit: Option<crate::datadogV1::model::NumberFormatUnit>,
    /// The definition of `NumberFormatUnitScale` object.
    #[serde(
        rename = "unit_scale",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub unit_scale: Option<Option<crate::datadogV1::model::NumberFormatUnitScale>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl WidgetNumberFormat {
    pub fn new() -> WidgetNumberFormat {
        WidgetNumberFormat {
            unit: None,
            unit_scale: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn unit(mut self, value: crate::datadogV1::model::NumberFormatUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn unit_scale(
        mut self,
        value: Option<crate::datadogV1::model::NumberFormatUnitScale>,
    ) -> Self {
        self.unit_scale = Some(value);
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

impl Default for WidgetNumberFormat {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for WidgetNumberFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct WidgetNumberFormatVisitor;
        impl<'a> Visitor<'a> for WidgetNumberFormatVisitor {
            type Value = WidgetNumberFormat;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut unit: Option<crate::datadogV1::model::NumberFormatUnit> = None;
                let mut unit_scale: Option<Option<crate::datadogV1::model::NumberFormatUnitScale>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "unit" => {
                            if v.is_null() {
                                continue;
                            }
                            unit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _unit) = unit {
                                match _unit {
                                    crate::datadogV1::model::NumberFormatUnit::UnparsedObject(
                                        _unit,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "unit_scale" => {
                            unit_scale = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = WidgetNumberFormat {
                    unit,
                    unit_scale,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(WidgetNumberFormatVisitor)
    }
}
