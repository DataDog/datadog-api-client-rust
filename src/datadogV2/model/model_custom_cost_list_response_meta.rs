// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Meta for the response from the List Custom Costs endpoints.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomCostListResponseMeta {
    /// Number of Custom Costs files per status.
    #[serde(rename = "count_by_status")]
    pub count_by_status: Option<std::collections::BTreeMap<String, i64>>,
    /// List of available providers.
    #[serde(rename = "providers")]
    pub providers: Option<Vec<String>>,
    /// Number of Custom Costs files returned by the List Custom Costs endpoint
    #[serde(rename = "total_filtered_count")]
    pub total_filtered_count: Option<i64>,
    /// Version of Custom Costs file
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomCostListResponseMeta {
    pub fn new() -> CustomCostListResponseMeta {
        CustomCostListResponseMeta {
            count_by_status: None,
            providers: None,
            total_filtered_count: None,
            version: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn count_by_status(mut self, value: std::collections::BTreeMap<String, i64>) -> Self {
        self.count_by_status = Some(value);
        self
    }

    pub fn providers(mut self, value: Vec<String>) -> Self {
        self.providers = Some(value);
        self
    }

    pub fn total_filtered_count(mut self, value: i64) -> Self {
        self.total_filtered_count = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
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

impl Default for CustomCostListResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomCostListResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomCostListResponseMetaVisitor;
        impl<'a> Visitor<'a> for CustomCostListResponseMetaVisitor {
            type Value = CustomCostListResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut count_by_status: Option<std::collections::BTreeMap<String, i64>> = None;
                let mut providers: Option<Vec<String>> = None;
                let mut total_filtered_count: Option<i64> = None;
                let mut version: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "count_by_status" => {
                            if v.is_null() {
                                continue;
                            }
                            count_by_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "providers" => {
                            if v.is_null() {
                                continue;
                            }
                            providers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_filtered_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_filtered_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CustomCostListResponseMeta {
                    count_by_status,
                    providers,
                    total_filtered_count,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomCostListResponseMetaVisitor)
    }
}
