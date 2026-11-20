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
pub struct TreemapBackendRequest20270101OneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "dataset_to_query")]
    pub dataset_to_query: String,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "datasets")]
    pub datasets: Vec<crate::datadogV1_20270101::model::AdvancedQueryDataset>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: Option<crate::datadogV1_20270101::model::TabularProjection>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::TreemapBackendRequestAnyOfAnyOfItemRequestType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::TreemapBackendRequestAnyOfAnyOfItemResponseFormat,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::TreemapRequestStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl TreemapBackendRequest20270101OneOf {
    pub fn new(dataset_to_query: String, datasets: Vec<crate::datadogV1_20270101::model::AdvancedQueryDataset>, request_type: crate::datadogV1_20270101::model::TreemapBackendRequestAnyOfAnyOfItemRequestType, response_format: crate::datadogV1_20270101::model::TreemapBackendRequestAnyOfAnyOfItemResponseFormat) -> TreemapBackendRequest20270101OneOf {
        TreemapBackendRequest20270101OneOf {
            dataset_to_query,
            datasets,
            limit: None,
            projection: None,
            request_type,
            response_format,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn projection(mut self, value: crate::datadogV1_20270101::model::TabularProjection) -> Self {
        self.projection = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::TreemapRequestStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for TreemapBackendRequest20270101OneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TreemapBackendRequest20270101OneOfVisitor;
        impl<'a> Visitor<'a> for TreemapBackendRequest20270101OneOfVisitor {
            type Value = TreemapBackendRequest20270101OneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut dataset_to_query: Option<String> = None;
                let mut datasets: Option<Vec<crate::datadogV1_20270101::model::AdvancedQueryDataset>> = None;
                let mut limit: Option<f64> = None;
                let mut projection: Option<crate::datadogV1_20270101::model::TabularProjection> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::TreemapBackendRequestAnyOfAnyOfItemRequestType> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::TreemapBackendRequestAnyOfAnyOfItemResponseFormat> = None;
                let mut style: Option<crate::datadogV1_20270101::model::TreemapRequestStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "projection" => {
                            if v.is_null() {
                                continue;
                            }
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _projection) = projection {
                                match _projection {
                                    crate::datadogV1_20270101::model::TabularProjection::UnparsedObject(_projection) => {
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
                                    crate::datadogV1_20270101::model::TreemapBackendRequestAnyOfAnyOfItemRequestType::UnparsedObject(_request_type) => {
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
                                    crate::datadogV1_20270101::model::TreemapBackendRequestAnyOfAnyOfItemResponseFormat::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "style" => {
                            if v.is_null() {
                                continue;
                            }
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = TreemapBackendRequest20270101OneOf {
                    dataset_to_query,
                    datasets,
                    limit,
                    projection,
                    request_type,
                    response_format,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TreemapBackendRequest20270101OneOfVisitor)
    }
}
