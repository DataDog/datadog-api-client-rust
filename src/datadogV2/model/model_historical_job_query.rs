// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query for selecting logs analyzed by the historical job.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HistoricalJobQuery {
    /// Additional filters appended to the query at evaluation time.
    #[serde(rename = "additionalFilters")]
    pub additional_filters: Option<String>,
    /// The aggregation type.
    #[serde(rename = "aggregation")]
    pub aggregation: Option<crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation>,
    /// Fields used to correlate results across queries in sequence detection rules.
    #[serde(rename = "correlatedByFields")]
    pub correlated_by_fields: Option<Vec<String>>,
    /// Zero-based index of the query to correlate with in sequence detection rules. Up to 10 queries are supported, so valid values are 0 to 9.
    #[serde(rename = "correlatedQueryIndex")]
    pub correlated_query_index: Option<i64>,
    /// Custom query extension used to refine the base query.
    #[serde(rename = "customQueryExtension")]
    pub custom_query_extension: Option<String>,
    /// Source of events, either logs, audit trail, security signals, or Datadog events. `app_sec_spans` is deprecated in favor of `spans`.
    #[serde(rename = "dataSource")]
    pub data_source: Option<crate::datadogV2::model::SecurityMonitoringStandardDataSource>,
    /// IDs of reference datasets used by this query.
    #[serde(rename = "datasetIds")]
    pub dataset_ids: Option<Vec<String>>,
    /// Field for which the cardinality is measured. Sent as an array.
    #[serde(rename = "distinctFields")]
    pub distinct_fields: Option<Vec<String>>,
    /// Fields to group by.
    #[serde(rename = "groupByFields")]
    pub group_by_fields: Option<Vec<String>>,
    /// When false, events without a group-by value are ignored by the query. When true, events with missing group-by fields are processed with `N/A`, replacing the missing values.
    #[serde(rename = "hasOptionalGroupByFields")]
    pub has_optional_group_by_fields: Option<bool>,
    /// Index used to load the data for this query.
    #[serde(rename = "index")]
    pub index: Option<String>,
    /// Indexes used to load the data for this query. Mutually exclusive with `index`.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Group of target fields to aggregate over when using the sum, max, geo data, or new value aggregations. The sum, max, and geo data aggregations only accept one value in this list, whereas the new value aggregation accepts up to five values.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    /// Name of the query.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Query to run on logs.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Language used to parse the query string.
    #[serde(rename = "queryLanguage")]
    pub query_language: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HistoricalJobQuery {
    pub fn new() -> HistoricalJobQuery {
        HistoricalJobQuery {
            additional_filters: None,
            aggregation: None,
            correlated_by_fields: None,
            correlated_query_index: None,
            custom_query_extension: None,
            data_source: None,
            dataset_ids: None,
            distinct_fields: None,
            group_by_fields: None,
            has_optional_group_by_fields: None,
            index: None,
            indexes: None,
            metrics: None,
            name: None,
            query: None,
            query_language: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_filters(mut self, value: String) -> Self {
        self.additional_filters = Some(value);
        self
    }

    pub fn aggregation(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation,
    ) -> Self {
        self.aggregation = Some(value);
        self
    }

    pub fn correlated_by_fields(mut self, value: Vec<String>) -> Self {
        self.correlated_by_fields = Some(value);
        self
    }

    pub fn correlated_query_index(mut self, value: i64) -> Self {
        self.correlated_query_index = Some(value);
        self
    }

    pub fn custom_query_extension(mut self, value: String) -> Self {
        self.custom_query_extension = Some(value);
        self
    }

    pub fn data_source(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringStandardDataSource,
    ) -> Self {
        self.data_source = Some(value);
        self
    }

    pub fn dataset_ids(mut self, value: Vec<String>) -> Self {
        self.dataset_ids = Some(value);
        self
    }

    pub fn distinct_fields(mut self, value: Vec<String>) -> Self {
        self.distinct_fields = Some(value);
        self
    }

    pub fn group_by_fields(mut self, value: Vec<String>) -> Self {
        self.group_by_fields = Some(value);
        self
    }

    pub fn has_optional_group_by_fields(mut self, value: bool) -> Self {
        self.has_optional_group_by_fields = Some(value);
        self
    }

    pub fn index(mut self, value: String) -> Self {
        self.index = Some(value);
        self
    }

    pub fn indexes(mut self, value: Vec<String>) -> Self {
        self.indexes = Some(value);
        self
    }

    pub fn metrics(mut self, value: Vec<String>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn query_language(mut self, value: String) -> Self {
        self.query_language = Some(value);
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

impl Default for HistoricalJobQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HistoricalJobQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HistoricalJobQueryVisitor;
        impl<'a> Visitor<'a> for HistoricalJobQueryVisitor {
            type Value = HistoricalJobQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut additional_filters: Option<String> = None;
                let mut aggregation: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation,
                > = None;
                let mut correlated_by_fields: Option<Vec<String>> = None;
                let mut correlated_query_index: Option<i64> = None;
                let mut custom_query_extension: Option<String> = None;
                let mut data_source: Option<
                    crate::datadogV2::model::SecurityMonitoringStandardDataSource,
                > = None;
                let mut dataset_ids: Option<Vec<String>> = None;
                let mut distinct_fields: Option<Vec<String>> = None;
                let mut group_by_fields: Option<Vec<String>> = None;
                let mut has_optional_group_by_fields: Option<bool> = None;
                let mut index: Option<String> = None;
                let mut indexes: Option<Vec<String>> = None;
                let mut metrics: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut query_language: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "additionalFilters" => {
                            if v.is_null() {
                                continue;
                            }
                            additional_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "aggregation" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregation) = aggregation {
                                match _aggregation {
                                    crate::datadogV2::model::SecurityMonitoringRuleQueryAggregation::UnparsedObject(_aggregation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "correlatedByFields" => {
                            if v.is_null() {
                                continue;
                            }
                            correlated_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "correlatedQueryIndex" => {
                            if v.is_null() {
                                continue;
                            }
                            correlated_query_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customQueryExtension" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_query_extension =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataSource" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::SecurityMonitoringStandardDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "datasetIds" => {
                            if v.is_null() {
                                continue;
                            }
                            dataset_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "distinctFields" => {
                            if v.is_null() {
                                continue;
                            }
                            distinct_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groupByFields" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hasOptionalGroupByFields" => {
                            if v.is_null() {
                                continue;
                            }
                            has_optional_group_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index" => {
                            if v.is_null() {
                                continue;
                            }
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            if v.is_null() {
                                continue;
                            }
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queryLanguage" => {
                            if v.is_null() {
                                continue;
                            }
                            query_language =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = HistoricalJobQuery {
                    additional_filters,
                    aggregation,
                    correlated_by_fields,
                    correlated_query_index,
                    custom_query_extension,
                    data_source,
                    dataset_ids,
                    distinct_fields,
                    group_by_fields,
                    has_optional_group_by_fields,
                    index,
                    indexes,
                    metrics,
                    name,
                    query,
                    query_language,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HistoricalJobQueryVisitor)
    }
}
