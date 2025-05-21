// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `OCIMetricsConfig` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OCIMetricsConfig {
    /// The compartment tag filters to apply to metric collection. Each value represents a Datadog tag in the format key:value.
    #[serde(rename = "compartment_tag_filters")]
    pub compartment_tag_filters: Option<Vec<String>>,
    /// Enable or disable metric collection. Enabled by default for all services.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The list of services to exclude from metric collection.
    #[serde(rename = "excluded_services")]
    pub excluded_services: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OCIMetricsConfig {
    pub fn new() -> OCIMetricsConfig {
        OCIMetricsConfig {
            compartment_tag_filters: None,
            enabled: None,
            excluded_services: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn compartment_tag_filters(mut self, value: Vec<String>) -> Self {
        self.compartment_tag_filters = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn excluded_services(mut self, value: Vec<String>) -> Self {
        self.excluded_services = Some(value);
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

impl Default for OCIMetricsConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OCIMetricsConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OCIMetricsConfigVisitor;
        impl<'a> Visitor<'a> for OCIMetricsConfigVisitor {
            type Value = OCIMetricsConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compartment_tag_filters: Option<Vec<String>> = None;
                let mut enabled: Option<bool> = None;
                let mut excluded_services: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compartment_tag_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            compartment_tag_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "excluded_services" => {
                            if v.is_null() {
                                continue;
                            }
                            excluded_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OCIMetricsConfig {
                    compartment_tag_filters,
                    enabled,
                    excluded_services,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OCIMetricsConfigVisitor)
    }
}
