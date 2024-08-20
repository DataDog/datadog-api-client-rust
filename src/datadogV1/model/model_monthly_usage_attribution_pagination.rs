// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The metadata for the current pagination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonthlyUsageAttributionPagination {
    /// The cursor to use to get the next results, if any. To make the next request, use the same parameters with the addition of the `next_record_id`.
    #[serde(
        rename = "next_record_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_record_id: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonthlyUsageAttributionPagination {
    pub fn new() -> MonthlyUsageAttributionPagination {
        MonthlyUsageAttributionPagination {
            next_record_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn next_record_id(mut self, value: Option<String>) -> Self {
        self.next_record_id = Some(value);
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

impl Default for MonthlyUsageAttributionPagination {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonthlyUsageAttributionPagination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonthlyUsageAttributionPaginationVisitor;
        impl<'a> Visitor<'a> for MonthlyUsageAttributionPaginationVisitor {
            type Value = MonthlyUsageAttributionPagination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut next_record_id: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "next_record_id" => {
                            next_record_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonthlyUsageAttributionPagination {
                    next_record_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonthlyUsageAttributionPaginationVisitor)
    }
}
