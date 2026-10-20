// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// UUID cursor pagination. A full final page can return a cursor followed by an empty page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TemporaryGrantRequestPage {
    /// Use this UUID as page[cursor] to retrieve the next page. Omitted when the page is not full.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<uuid::Uuid>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TemporaryGrantRequestPage {
    pub fn new() -> TemporaryGrantRequestPage {
        TemporaryGrantRequestPage {
            next_cursor: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn next_cursor(mut self, value: uuid::Uuid) -> Self {
        self.next_cursor = Some(value);
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

impl Default for TemporaryGrantRequestPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TemporaryGrantRequestPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporaryGrantRequestPageVisitor;
        impl<'a> Visitor<'a> for TemporaryGrantRequestPageVisitor {
            type Value = TemporaryGrantRequestPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut next_cursor: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "next_cursor" => {
                            if v.is_null() {
                                continue;
                            }
                            next_cursor =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TemporaryGrantRequestPage {
                    next_cursor,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TemporaryGrantRequestPageVisitor)
    }
}
