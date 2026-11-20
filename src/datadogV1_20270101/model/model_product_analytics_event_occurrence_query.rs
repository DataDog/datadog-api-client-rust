// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsEventOccurrenceQuery {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1_20270101::model::ProductAnalyticsEventOccurrenceQueryDataSource,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "search")]
    pub search: crate::datadogV1_20270101::model::ProductAnalyticsEventOccurrenceQuerySearch,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ProductAnalyticsEventOccurrenceQuery {
    pub fn new(data_source: crate::datadogV1_20270101::model::ProductAnalyticsEventOccurrenceQueryDataSource, search: crate::datadogV1_20270101::model::ProductAnalyticsEventOccurrenceQuerySearch) -> ProductAnalyticsEventOccurrenceQuery {
        ProductAnalyticsEventOccurrenceQuery {
            data_source,
            search,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for ProductAnalyticsEventOccurrenceQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsEventOccurrenceQueryVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsEventOccurrenceQueryVisitor {
            type Value = ProductAnalyticsEventOccurrenceQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<crate::datadogV1_20270101::model::ProductAnalyticsEventOccurrenceQueryDataSource> = None;
                let mut search: Option<crate::datadogV1_20270101::model::ProductAnalyticsEventOccurrenceQuerySearch> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1_20270101::model::ProductAnalyticsEventOccurrenceQueryDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let data_source = data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = ProductAnalyticsEventOccurrenceQuery {
                    data_source,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsEventOccurrenceQueryVisitor)
    }
}
