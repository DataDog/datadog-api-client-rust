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
pub struct ScatterplotBackendTableRequest20270101OneOf4461525600 {
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
    pub projection: Option<crate::datadogV1_20270101::model::ScatterplotProjection>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451373184,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "response_format")]
    pub response_format: crate::datadogV1_20270101::model::ScatterplotBackendTableRequestAnyOfAnyOfItemResponseFormat4451373984,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScatterplotBackendTableRequest20270101OneOf4461525600 {
    pub fn new(dataset_to_query: String, datasets: Vec<crate::datadogV1_20270101::model::AdvancedQueryDataset>, request_type: crate::datadogV1_20270101::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451373184, response_format: crate::datadogV1_20270101::model::ScatterplotBackendTableRequestAnyOfAnyOfItemResponseFormat4451373984) -> ScatterplotBackendTableRequest20270101OneOf4461525600 {
        ScatterplotBackendTableRequest20270101OneOf4461525600 {
            dataset_to_query,
            datasets,
            limit: None,
            projection: None,
            request_type,
            response_format,
            _unparsed: false,
        }
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn projection(mut self, value: crate::datadogV1_20270101::model::ScatterplotProjection) -> Self {
        self.projection = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ScatterplotBackendTableRequest20270101OneOf4461525600 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterplotBackendTableRequest20270101OneOf4461525600Visitor;
        impl<'a> Visitor<'a> for ScatterplotBackendTableRequest20270101OneOf4461525600Visitor {
            type Value = ScatterplotBackendTableRequest20270101OneOf4461525600;

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
                let mut projection: Option<crate::datadogV1_20270101::model::ScatterplotProjection> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451373184> = None;
                let mut response_format: Option<crate::datadogV1_20270101::model::ScatterplotBackendTableRequestAnyOfAnyOfItemResponseFormat4451373984> = None;
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
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1_20270101::model::ScatterplotBackendTableRequestAnyOfAnyOfItemRequestType4451373184::UnparsedObject(_request_type) => {
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
                                    crate::datadogV1_20270101::model::ScatterplotBackendTableRequestAnyOfAnyOfItemResponseFormat4451373984::UnparsedObject(_response_format) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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

                let content = ScatterplotBackendTableRequest20270101OneOf4461525600 {
                    dataset_to_query,
                    datasets,
                    limit,
                    projection,
                    request_type,
                    response_format,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterplotBackendTableRequest20270101OneOf4461525600Visitor)
    }
}
