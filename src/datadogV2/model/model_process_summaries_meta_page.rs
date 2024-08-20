// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Paging attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProcessSummariesMetaPage {
    /// The cursor used to get the next results, if any. To make the next request, use the same
    /// parameters with the addition of the `page[cursor]`.
    #[serde(rename = "after")]
    pub after: Option<String>,
    /// Number of results returned.
    #[serde(rename = "size")]
    pub size: Option<i32>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProcessSummariesMetaPage {
    pub fn new() -> ProcessSummariesMetaPage {
        ProcessSummariesMetaPage {
            after: None,
            size: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn after(mut self, value: String) -> Self {
        self.after = Some(value);
        self
    }

    pub fn size(mut self, value: i32) -> Self {
        self.size = Some(value);
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

impl Default for ProcessSummariesMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProcessSummariesMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProcessSummariesMetaPageVisitor;
        impl<'a> Visitor<'a> for ProcessSummariesMetaPageVisitor {
            type Value = ProcessSummariesMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut after: Option<String> = None;
                let mut size: Option<i32> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "after" => {
                            if v.is_null() {
                                continue;
                            }
                            after = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProcessSummariesMetaPage {
                    after,
                    size,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProcessSummariesMetaPageVisitor)
    }
}
