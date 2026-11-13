// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Resource usage and organization details for the requested month.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonthlyResourceAttributes {
    /// Date of the source usage data. May be empty when unavailable.
    #[serde(rename = "data_date")]
    pub data_date: Option<String>,
    /// Name of the organization that owns the resource.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// Public ID of the organization that owns the resource.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Name of the resource, such as a host name, metric name, or log index.
    #[serde(rename = "resource_name")]
    pub resource_name: Option<String>,
    /// Resource tags when available. Empty for resource types without tag data.
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// Selected billing hour for percentile usage, or the first day of the month for monthly usage. May be empty when unavailable.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,
    /// Usage type of the resource. Values depend on the billing dimension and are not limited to host types.
    #[serde(rename = "usage_type")]
    pub usage_type: Option<String>,
    /// Usage attributed to the resource in the units of the requested billing dimension. Values can be fractional.
    #[serde(
        rename = "usage_value",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub usage_value: Option<Option<f64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonthlyResourceAttributes {
    pub fn new() -> MonthlyResourceAttributes {
        MonthlyResourceAttributes {
            data_date: None,
            org_name: None,
            public_id: None,
            resource_name: None,
            tags: None,
            timestamp: None,
            usage_type: None,
            usage_value: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data_date(mut self, value: String) -> Self {
        self.data_date = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn resource_name(mut self, value: String) -> Self {
        self.resource_name = Some(value);
        self
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp(mut self, value: String) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn usage_type(mut self, value: String) -> Self {
        self.usage_type = Some(value);
        self
    }

    pub fn usage_value(mut self, value: Option<f64>) -> Self {
        self.usage_value = Some(value);
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

impl Default for MonthlyResourceAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonthlyResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonthlyResourceAttributesVisitor;
        impl<'a> Visitor<'a> for MonthlyResourceAttributesVisitor {
            type Value = MonthlyResourceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_date: Option<String> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut resource_name: Option<String> = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut timestamp: Option<String> = None;
                let mut usage_type: Option<String> = None;
                let mut usage_value: Option<Option<f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_date" => {
                            if v.is_null() {
                                continue;
                            }
                            data_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_name" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_type" => {
                            if v.is_null() {
                                continue;
                            }
                            usage_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage_value" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            usage_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonthlyResourceAttributes {
                    data_date,
                    org_name,
                    public_id,
                    resource_name,
                    tags,
                    timestamp,
                    usage_type,
                    usage_value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonthlyResourceAttributesVisitor)
    }
}
