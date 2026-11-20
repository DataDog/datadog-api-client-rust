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
pub struct QueryTableBackendRequest20270101OneOf4460516096 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "aggregator")]
    pub aggregator: Option<crate::datadogV1::model::ScalarAggregator>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<Vec<crate::datadogV1::model::CellDisplayModeOption>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dataset_to_query")]
    pub dataset_to_query: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "datasets")]
    pub datasets: Vec<crate::datadogV1::model::AdvancedQueryDataset>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1::model::QuerySortOrder>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: Option<crate::datadogV1::model::TabularProjection>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1::model::QueryTableBackendRequestAnyOfAnyOfItemRequestType4450135840,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1::model::QueryTableBackendRequestAnyOfAnyOfItemResponseFormat4450136640,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "text_formats")]
    pub text_formats: Option<Vec<Vec<crate::datadogV1::model::TextFormattingRule>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl QueryTableBackendRequest20270101OneOf4460516096 {
    pub fn new(dataset_to_query: String, datasets: Vec<crate::datadogV1::model::AdvancedQueryDataset>, request_type: crate::datadogV1::model::QueryTableBackendRequestAnyOfAnyOfItemRequestType4450135840, response_format: crate::datadogV1::model::QueryTableBackendRequestAnyOfAnyOfItemResponseFormat4450136640) -> QueryTableBackendRequest20270101OneOf4460516096 {
        QueryTableBackendRequest20270101OneOf4460516096 {
            aggregator: None,
            alias: None,
            cell_display_mode: None,
            conditional_formats: None,
            dataset_to_query,
            datasets,
            limit: None,
            order: None,
            projection: None,
            request_type,
            response_format,
            text_formats: None,
            _unparsed: false,
        }
    }
    
    pub fn aggregator(mut self, value: crate::datadogV1::model::ScalarAggregator) -> Self {
        self.aggregator = Some(value);
        self
    }
    
    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }
    
    pub fn cell_display_mode(mut self, value: Vec<crate::datadogV1::model::CellDisplayModeOption>) -> Self {
        self.cell_display_mode = Some(value);
        self
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn order(mut self, value: crate::datadogV1::model::QuerySortOrder) -> Self {
        self.order = Some(value);
        self
    }
    
    pub fn projection(mut self, value: crate::datadogV1::model::TabularProjection) -> Self {
        self.projection = Some(value);
        self
    }
    
    pub fn text_formats(mut self, value: Vec<Vec<crate::datadogV1::model::TextFormattingRule>>) -> Self {
        self.text_formats = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for QueryTableBackendRequest20270101OneOf4460516096 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QueryTableBackendRequest20270101OneOf4460516096Visitor;
        impl<'a> Visitor<'a> for QueryTableBackendRequest20270101OneOf4460516096Visitor {
            type Value = QueryTableBackendRequest20270101OneOf4460516096;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregator: Option<crate::datadogV1::model::ScalarAggregator> = None;
                let mut alias: Option<String> = None;
                let mut cell_display_mode: Option<Vec<crate::datadogV1::model::CellDisplayModeOption>> = None;
                let mut conditional_formats: Option<Vec<crate::datadogV1::model::ThresholdFormattingRule>> = None;
                let mut dataset_to_query: Option<String> = None;
                let mut datasets: Option<Vec<crate::datadogV1::model::AdvancedQueryDataset>> = None;
                let mut limit: Option<f64> = None;
                let mut order: Option<crate::datadogV1::model::QuerySortOrder> = None;
                let mut projection: Option<crate::datadogV1::model::TabularProjection> = None;
                let mut request_type: Option<crate::datadogV1::model::QueryTableBackendRequestAnyOfAnyOfItemRequestType4450135840> = None;
                let mut response_format: Option<crate::datadogV1::model::QueryTableBackendRequestAnyOfAnyOfItemResponseFormat4450136640> = None;
                let mut text_formats: Option<Vec<Vec<crate::datadogV1::model::TextFormattingRule>>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregator" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _aggregator) = aggregator {
                                match _aggregator {
                                    crate::datadogV1::model::ScalarAggregator::UnparsedObject(_aggregator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "alias" => {
                            if v.is_null() {
                                continue;
                            }
                            alias = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "cell_display_mode" => {
                            if v.is_null() {
                                continue;
                            }
                            cell_display_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "dataset_to_query" => {
                            dataset_to_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "datasets" => {
                            datasets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order" => {
                            if v.is_null() {
                                continue;
                            }
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1::model::QuerySortOrder::UnparsedObject(_order) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "projection" => {
                            if v.is_null() {
                                continue;
                            }
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _projection) = projection {
                                match _projection {
                                    crate::datadogV1::model::TabularProjection::UnparsedObject(_projection) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1::model::QueryTableBackendRequestAnyOfAnyOfItemRequestType4450135840::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "response_format" => {
                            response_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _response_format) = response_format {
                                match _response_format {
                                    crate::datadogV1::model::QueryTableBackendRequestAnyOfAnyOfItemResponseFormat4450136640::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "text_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            text_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let dataset_to_query = dataset_to_query.ok_or_else(|| M::Error::missing_field("dataset_to_query"))?;
                let datasets = datasets.ok_or_else(|| M::Error::missing_field("datasets"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;
                let response_format = response_format.ok_or_else(|| M::Error::missing_field("response_format"))?;

                let content = QueryTableBackendRequest20270101OneOf4460516096 {
                    aggregator,
                    alias,
                    cell_display_mode,
                    conditional_formats,
                    dataset_to_query,
                    datasets,
                    limit,
                    order,
                    projection,
                    request_type,
                    response_format,
                    text_formats,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(QueryTableBackendRequest20270101OneOf4460516096Visitor)
    }
}
