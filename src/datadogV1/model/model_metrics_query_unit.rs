// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the metric unit family, scale factor, name, and short name.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricsQueryUnit {
    /// Unit family, allows for conversion between units of the same family, for scaling.
    #[serde(rename = "family")]
    pub family: Option<String>,
    /// Unit name
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Plural form of the unit name.
    #[serde(rename = "plural")]
    pub plural: Option<String>,
    /// Factor for scaling between units of the same family.
    #[serde(rename = "scale_factor")]
    pub scale_factor: Option<f64>,
    /// Abbreviation of the unit.
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricsQueryUnit {
    pub fn new() -> MetricsQueryUnit {
        MetricsQueryUnit {
            family: None,
            name: None,
            plural: None,
            scale_factor: None,
            short_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn family(mut self, value: String) -> Self {
        self.family = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn plural(mut self, value: String) -> Self {
        self.plural = Some(value);
        self
    }

    pub fn scale_factor(mut self, value: f64) -> Self {
        self.scale_factor = Some(value);
        self
    }

    pub fn short_name(mut self, value: String) -> Self {
        self.short_name = Some(value);
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

impl Default for MetricsQueryUnit {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricsQueryUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricsQueryUnitVisitor;
        impl<'a> Visitor<'a> for MetricsQueryUnitVisitor {
            type Value = MetricsQueryUnit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut family: Option<String> = None;
                let mut name: Option<String> = None;
                let mut plural: Option<String> = None;
                let mut scale_factor: Option<f64> = None;
                let mut short_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "family" => {
                            if v.is_null() {
                                continue;
                            }
                            family = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "plural" => {
                            if v.is_null() {
                                continue;
                            }
                            plural = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scale_factor" => {
                            if v.is_null() {
                                continue;
                            }
                            scale_factor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_name" => {
                            if v.is_null() {
                                continue;
                            }
                            short_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MetricsQueryUnit {
                    family,
                    name,
                    plural,
                    scale_factor,
                    short_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricsQueryUnitVisitor)
    }
}
