// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Count of integrations by type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationCounts {
    /// The filtered count of integrations.
    #[serde(rename = "filtered_count")]
    pub filtered_count: Option<i64>,
    /// The integration type.
    #[serde(rename = "integration_type")]
    pub integration_type: Option<String>,
    /// The total count of integrations.
    #[serde(rename = "total_count")]
    pub total_count: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationCounts {
    pub fn new() -> IntegrationCounts {
        IntegrationCounts {
            filtered_count: None,
            integration_type: None,
            total_count: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filtered_count(mut self, value: i64) -> Self {
        self.filtered_count = Some(value);
        self
    }

    pub fn integration_type(mut self, value: String) -> Self {
        self.integration_type = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
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

impl Default for IntegrationCounts {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationCounts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationCountsVisitor;
        impl<'a> Visitor<'a> for IntegrationCountsVisitor {
            type Value = IntegrationCounts;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filtered_count: Option<i64> = None;
                let mut integration_type: Option<String> = None;
                let mut total_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filtered_count" => {
                            if v.is_null() {
                                continue;
                            }
                            filtered_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_type" => {
                            if v.is_null() {
                                continue;
                            }
                            integration_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_count" => {
                            if v.is_null() {
                                continue;
                            }
                            total_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationCounts {
                    filtered_count,
                    integration_type,
                    total_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationCountsVisitor)
    }
}
