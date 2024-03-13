// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An individual scalar metrics query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricsScalarQuery {
    /// The type of aggregation that can be performed on metrics-based queries.
    #[serde(rename = "aggregator")]
    pub aggregator: crate::datadogV2::model::MetricsAggregator,
    /// A data source that is powered by the Metrics platform.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV2::model::MetricsDataSource,
    /// The variable name for use in formulas.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A classic metrics query string.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricsScalarQuery {
    pub fn new(
        aggregator: crate::datadogV2::model::MetricsAggregator,
        data_source: crate::datadogV2::model::MetricsDataSource,
        query: String,
    ) -> MetricsScalarQuery {
        MetricsScalarQuery {
            aggregator,
            data_source,
            name: None,
            query,
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MetricsScalarQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricsScalarQueryVisitor;
        impl<'a> Visitor<'a> for MetricsScalarQueryVisitor {
            type Value = MetricsScalarQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV2::model::MetricsAggregator> = None;
                let mut data_source: Option<crate::datadogV2::model::MetricsDataSource> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregator" => {
                            aggregator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregator) = aggregator {
                                match _aggregator {
                                    crate::datadogV2::model::MetricsAggregator::UnparsedObject(
                                        _aggregator,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::MetricsDataSource::UnparsedObject(
                                        _data_source,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let aggregator = aggregator.ok_or_else(|| M::Error::missing_field("aggregator"))?;
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = MetricsScalarQuery {
                    aggregator,
                    data_source,
                    name,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricsScalarQueryVisitor)
    }
}
