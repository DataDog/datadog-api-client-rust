// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query for available values using the metrics data source.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardAvailableValuesMetricsQuery {
    /// Data source for metrics queries.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FormulaAndFunctionMetricDataSource,
    /// The metrics query string.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardAvailableValuesMetricsQuery {
    pub fn new(
        data_source: crate::datadogV1::model::FormulaAndFunctionMetricDataSource,
        query: String,
    ) -> DashboardAvailableValuesMetricsQuery {
        DashboardAvailableValuesMetricsQuery {
            data_source,
            query,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for DashboardAvailableValuesMetricsQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardAvailableValuesMetricsQueryVisitor;
        impl<'a> Visitor<'a> for DashboardAvailableValuesMetricsQueryVisitor {
            type Value = DashboardAvailableValuesMetricsQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<
                    crate::datadogV1::model::FormulaAndFunctionMetricDataSource,
                > = None;
                let mut query: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV1::model::FormulaAndFunctionMetricDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = DashboardAvailableValuesMetricsQuery {
                    data_source,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardAvailableValuesMetricsQueryVisitor)
    }
}
