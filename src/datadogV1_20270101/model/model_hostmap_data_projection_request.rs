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
pub struct HostmapDataProjectionRequest {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "conditional_formats")]
    pub conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "limit")]
    pub limit: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "no_group_hosts")]
    pub no_group_hosts: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "no_metric_hosts")]
    pub no_metric_hosts: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "projection")]
    pub projection: crate::datadogV1_20270101::model::HostmapProjection,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "query")]
    pub query: crate::datadogV1_20270101::model::DatasetListQuery,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "request_type")]
    pub request_type: crate::datadogV1_20270101::model::HostmapDataProjectionRequestRequestType,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "style")]
    pub style: Option<crate::datadogV1_20270101::model::HostmapStyle>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl HostmapDataProjectionRequest {
    pub fn new(projection: crate::datadogV1_20270101::model::HostmapProjection, query: crate::datadogV1_20270101::model::DatasetListQuery, request_type: crate::datadogV1_20270101::model::HostmapDataProjectionRequestRequestType) -> HostmapDataProjectionRequest {
        HostmapDataProjectionRequest {
            conditional_formats: None,
            limit: None,
            no_group_hosts: None,
            no_metric_hosts: None,
            projection,
            query,
            request_type,
            style: None,
            _unparsed: false,
        }
    }
    
    pub fn conditional_formats(mut self, value: Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>) -> Self {
        self.conditional_formats = Some(value);
        self
    }
    
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }
    
    pub fn no_group_hosts(mut self, value: bool) -> Self {
        self.no_group_hosts = Some(value);
        self
    }
    
    pub fn no_metric_hosts(mut self, value: bool) -> Self {
        self.no_metric_hosts = Some(value);
        self
    }
    
    pub fn style(mut self, value: crate::datadogV1_20270101::model::HostmapStyle) -> Self {
        self.style = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for HostmapDataProjectionRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HostmapDataProjectionRequestVisitor;
        impl<'a> Visitor<'a> for HostmapDataProjectionRequestVisitor {
            type Value = HostmapDataProjectionRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditional_formats: Option<Vec<crate::datadogV1_20270101::model::ThresholdFormattingRule>> = None;
                let mut limit: Option<f64> = None;
                let mut no_group_hosts: Option<bool> = None;
                let mut no_metric_hosts: Option<bool> = None;
                let mut projection: Option<crate::datadogV1_20270101::model::HostmapProjection> = None;
                let mut query: Option<crate::datadogV1_20270101::model::DatasetListQuery> = None;
                let mut request_type: Option<crate::datadogV1_20270101::model::HostmapDataProjectionRequestRequestType> = None;
                let mut style: Option<crate::datadogV1_20270101::model::HostmapStyle> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conditional_formats" => {
                            if v.is_null() {
                                continue;
                            }
                            conditional_formats = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "limit" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "no_group_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            no_group_hosts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "no_metric_hosts" => {
                            if v.is_null() {
                                continue;
                            }
                            no_metric_hosts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "projection" => {
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "request_type" => {
                            request_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV1_20270101::model::HostmapDataProjectionRequestRequestType::UnparsedObject(_request_type) => {
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
                let projection = projection.ok_or_else(|| M::Error::missing_field("projection"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let request_type = request_type.ok_or_else(|| M::Error::missing_field("request_type"))?;

                let content = HostmapDataProjectionRequest {
                    conditional_formats,
                    limit,
                    no_group_hosts,
                    no_metric_hosts,
                    projection,
                    query,
                    request_type,
                    style,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HostmapDataProjectionRequestVisitor)
    }
}
