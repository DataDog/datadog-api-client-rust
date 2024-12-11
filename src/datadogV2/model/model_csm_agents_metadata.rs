// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata related to the paginated response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CSMAgentsMetadata {
    /// The index of the current page in the paginated results.
    #[serde(rename = "page_index")]
    pub page_index: Option<i64>,
    /// The number of items per page in the paginated results.
    #[serde(rename = "page_size")]
    pub page_size: Option<i64>,
    /// Total number of items that match the filter criteria.
    #[serde(rename = "total_filtered")]
    pub total_filtered: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CSMAgentsMetadata {
    pub fn new() -> CSMAgentsMetadata {
        CSMAgentsMetadata {
            page_index: None,
            page_size: None,
            total_filtered: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn page_index(mut self, value: i64) -> Self {
        self.page_index = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total_filtered(mut self, value: i64) -> Self {
        self.total_filtered = Some(value);
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

impl Default for CSMAgentsMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CSMAgentsMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CSMAgentsMetadataVisitor;
        impl<'a> Visitor<'a> for CSMAgentsMetadataVisitor {
            type Value = CSMAgentsMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page_index: Option<i64> = None;
                let mut page_size: Option<i64> = None;
                let mut total_filtered: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page_index" => {
                            if v.is_null() {
                                continue;
                            }
                            page_index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "page_size" => {
                            if v.is_null() {
                                continue;
                            }
                            page_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_filtered" => {
                            if v.is_null() {
                                continue;
                            }
                            total_filtered =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CSMAgentsMetadata {
                    page_index,
                    page_size,
                    total_filtered,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CSMAgentsMetadataVisitor)
    }
}
