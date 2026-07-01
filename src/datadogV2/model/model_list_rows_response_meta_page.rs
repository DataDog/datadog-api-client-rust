// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Contains the continuation token for navigating to the next page of rows.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListRowsResponseMetaPage {
    /// Opaque token to pass as the `page[continuation_token]` query parameter to fetch the next page of results. Only present when more rows are available.
    #[serde(rename = "next_continuation_token")]
    pub next_continuation_token: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListRowsResponseMetaPage {
    pub fn new() -> ListRowsResponseMetaPage {
        ListRowsResponseMetaPage {
            next_continuation_token: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn next_continuation_token(mut self, value: String) -> Self {
        self.next_continuation_token = Some(value);
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

impl Default for ListRowsResponseMetaPage {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListRowsResponseMetaPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListRowsResponseMetaPageVisitor;
        impl<'a> Visitor<'a> for ListRowsResponseMetaPageVisitor {
            type Value = ListRowsResponseMetaPage;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut next_continuation_token: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "next_continuation_token" => {
                            if v.is_null() {
                                continue;
                            }
                            next_continuation_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ListRowsResponseMetaPage {
                    next_continuation_token,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListRowsResponseMetaPageVisitor)
    }
}
