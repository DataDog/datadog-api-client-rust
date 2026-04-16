// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Geographic location information for an IP indicator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IoCGeoLocation {
    /// City name.
    #[serde(rename = "city")]
    pub city: Option<String>,
    /// ISO country code.
    #[serde(rename = "country_code")]
    pub country_code: Option<String>,
    /// Full country name.
    #[serde(rename = "country_name")]
    pub country_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IoCGeoLocation {
    pub fn new() -> IoCGeoLocation {
        IoCGeoLocation {
            city: None,
            country_code: None,
            country_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn city(mut self, value: String) -> Self {
        self.city = Some(value);
        self
    }

    pub fn country_code(mut self, value: String) -> Self {
        self.country_code = Some(value);
        self
    }

    pub fn country_name(mut self, value: String) -> Self {
        self.country_name = Some(value);
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

impl Default for IoCGeoLocation {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IoCGeoLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IoCGeoLocationVisitor;
        impl<'a> Visitor<'a> for IoCGeoLocationVisitor {
            type Value = IoCGeoLocation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut city: Option<String> = None;
                let mut country_code: Option<String> = None;
                let mut country_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "city" => {
                            if v.is_null() {
                                continue;
                            }
                            city = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "country_code" => {
                            if v.is_null() {
                                continue;
                            }
                            country_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "country_name" => {
                            if v.is_null() {
                                continue;
                            }
                            country_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IoCGeoLocation {
                    city,
                    country_code,
                    country_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IoCGeoLocationVisitor)
    }
}
