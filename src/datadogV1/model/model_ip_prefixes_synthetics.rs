// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Available prefix information for the Synthetics endpoints.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IPPrefixesSynthetics {
    /// List of IPv4 prefixes.
    #[serde(rename = "prefixes_ipv4")]
    pub prefixes_ipv4: Option<Vec<String>>,
    /// List of IPv4 prefixes by location.
    #[serde(rename = "prefixes_ipv4_by_location")]
    pub prefixes_ipv4_by_location: Option<std::collections::BTreeMap<String, Vec<String>>>,
    /// List of IPv6 prefixes.
    #[serde(rename = "prefixes_ipv6")]
    pub prefixes_ipv6: Option<Vec<String>>,
    /// List of IPv6 prefixes by location.
    #[serde(rename = "prefixes_ipv6_by_location")]
    pub prefixes_ipv6_by_location: Option<std::collections::BTreeMap<String, Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IPPrefixesSynthetics {
    pub fn new() -> IPPrefixesSynthetics {
        IPPrefixesSynthetics {
            prefixes_ipv4: None,
            prefixes_ipv4_by_location: None,
            prefixes_ipv6: None,
            prefixes_ipv6_by_location: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn prefixes_ipv4(mut self, value: Vec<String>) -> Self {
        self.prefixes_ipv4 = Some(value);
        self
    }

    pub fn prefixes_ipv4_by_location(
        mut self,
        value: std::collections::BTreeMap<String, Vec<String>>,
    ) -> Self {
        self.prefixes_ipv4_by_location = Some(value);
        self
    }

    pub fn prefixes_ipv6(mut self, value: Vec<String>) -> Self {
        self.prefixes_ipv6 = Some(value);
        self
    }

    pub fn prefixes_ipv6_by_location(
        mut self,
        value: std::collections::BTreeMap<String, Vec<String>>,
    ) -> Self {
        self.prefixes_ipv6_by_location = Some(value);
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

impl Default for IPPrefixesSynthetics {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IPPrefixesSynthetics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IPPrefixesSyntheticsVisitor;
        impl<'a> Visitor<'a> for IPPrefixesSyntheticsVisitor {
            type Value = IPPrefixesSynthetics;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut prefixes_ipv4: Option<Vec<String>> = None;
                let mut prefixes_ipv4_by_location: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut prefixes_ipv6: Option<Vec<String>> = None;
                let mut prefixes_ipv6_by_location: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "prefixes_ipv4" => {
                            if v.is_null() {
                                continue;
                            }
                            prefixes_ipv4 =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefixes_ipv4_by_location" => {
                            if v.is_null() {
                                continue;
                            }
                            prefixes_ipv4_by_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefixes_ipv6" => {
                            if v.is_null() {
                                continue;
                            }
                            prefixes_ipv6 =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prefixes_ipv6_by_location" => {
                            if v.is_null() {
                                continue;
                            }
                            prefixes_ipv6_by_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IPPrefixesSynthetics {
                    prefixes_ipv4,
                    prefixes_ipv4_by_location,
                    prefixes_ipv6,
                    prefixes_ipv6_by_location,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IPPrefixesSyntheticsVisitor)
    }
}
