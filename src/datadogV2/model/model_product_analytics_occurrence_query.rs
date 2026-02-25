// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A Product Analytics occurrence-filtered query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsOccurrenceQuery {
    /// The data source identifier for occurrence queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::ProductAnalyticsOccurrenceQueryDataSource,
    /// Search parameters for an occurrence query.
    #[serde(rename = "search")]
    pub search: crate::datadogV2::model::ProductAnalyticsOccurrenceSearch,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsOccurrenceQuery {
    pub fn new(
        data_source: crate::datadogV2::model::ProductAnalyticsOccurrenceQueryDataSource,
        search: crate::datadogV2::model::ProductAnalyticsOccurrenceSearch,
    ) -> ProductAnalyticsOccurrenceQuery {
        ProductAnalyticsOccurrenceQuery {
            data_source,
            search,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsOccurrenceQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsOccurrenceQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsOccurrenceQueryVisitor {
            type Value = ProductAnalyticsOccurrenceQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<
                    crate::datadogV2::model::ProductAnalyticsOccurrenceQueryDataSource,
                > = None;
                let mut search: Option<crate::datadogV2::model::ProductAnalyticsOccurrenceSearch> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::ProductAnalyticsOccurrenceQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsOccurrenceQuery {
                    data_source,
                    search,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsOccurrenceQueryVisitor)
    }
}
