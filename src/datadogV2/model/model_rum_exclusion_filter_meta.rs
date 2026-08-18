// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the exclusion filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumExclusionFilterMeta {
    /// Unix epoch (in milliseconds) when the exclusion filter was last enabled.
    #[serde(rename = "enabled_at")]
    pub enabled_at: Option<i64>,
    /// Unix epoch (in milliseconds) of the last update.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<i64>,
    /// Handle of the user who last updated the exclusion filter.
    #[serde(rename = "updated_by_handle")]
    pub updated_by_handle: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumExclusionFilterMeta {
    pub fn new() -> RumExclusionFilterMeta {
        RumExclusionFilterMeta {
            enabled_at: None,
            updated_at: None,
            updated_by_handle: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled_at(mut self, value: i64) -> Self {
        self.enabled_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn updated_by_handle(mut self, value: String) -> Self {
        self.updated_by_handle = Some(value);
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

impl Default for RumExclusionFilterMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumExclusionFilterMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumExclusionFilterMetaVisitor;
        impl<'a> Visitor<'a> for RumExclusionFilterMetaVisitor {
            type Value = RumExclusionFilterMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled_at: Option<i64> = None;
                let mut updated_at: Option<i64> = None;
                let mut updated_by_handle: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled_at" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by_handle" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RumExclusionFilterMeta {
                    enabled_at,
                    updated_at,
                    updated_by_handle,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumExclusionFilterMetaVisitor)
    }
}
