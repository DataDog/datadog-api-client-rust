// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An array of service level objective objects.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOBulkDeleteResponseData {
    /// An array of service level objective object IDs that indicates
    /// which objects that were completely deleted.
    #[serde(rename = "deleted")]
    pub deleted: Option<Vec<String>>,
    /// An array of service level objective object IDs that indicates
    /// which objects that were modified (objects for which at least one
    /// threshold was deleted, but that were not completely deleted).
    #[serde(rename = "updated")]
    pub updated: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOBulkDeleteResponseData {
    pub fn new() -> SLOBulkDeleteResponseData {
        SLOBulkDeleteResponseData {
            deleted: None,
            updated: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn deleted(mut self, value: Vec<String>) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn updated(mut self, value: Vec<String>) -> Self {
        self.updated = Some(value);
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

impl Default for SLOBulkDeleteResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOBulkDeleteResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOBulkDeleteResponseDataVisitor;
        impl<'a> Visitor<'a> for SLOBulkDeleteResponseDataVisitor {
            type Value = SLOBulkDeleteResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted: Option<Vec<String>> = None;
                let mut updated: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated" => {
                            if v.is_null() {
                                continue;
                            }
                            updated = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SLOBulkDeleteResponseData {
                    deleted,
                    updated,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOBulkDeleteResponseDataVisitor)
    }
}
