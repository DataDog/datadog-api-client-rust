// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ListAppKeyRegistrationsResponseMeta` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListAppKeyRegistrationsResponseMeta {
    /// The total number of app key registrations.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    /// The total number of app key registrations that match the specified filters.
    #[serde(rename = "total_filtered")]
    pub total_filtered: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ListAppKeyRegistrationsResponseMeta {
    pub fn new() -> ListAppKeyRegistrationsResponseMeta {
        ListAppKeyRegistrationsResponseMeta {
            total: None,
            total_filtered: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
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

impl Default for ListAppKeyRegistrationsResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListAppKeyRegistrationsResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListAppKeyRegistrationsResponseMetaVisitor;
        impl<'a> Visitor<'a> for ListAppKeyRegistrationsResponseMetaVisitor {
            type Value = ListAppKeyRegistrationsResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut total: Option<i64> = None;
                let mut total_filtered: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "total" => {
                            if v.is_null() {
                                continue;
                            }
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ListAppKeyRegistrationsResponseMeta {
                    total,
                    total_filtered,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListAppKeyRegistrationsResponseMetaVisitor)
    }
}
